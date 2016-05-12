use nix;
use nix::sys::socket;
use nix::sys::uio::IoVec;
use nix::unistd;
use std::result;
use std::fmt;
use std::num;
use std::str;
use std::str::FromStr;

use std::os::unix::io::RawFd;

// OS X doesn't let us go beyond 256kB for the buffer size, so this is the max:
const SEND_BUF_SIZE: usize = 900 * 1024;

/// A ring buffer containing file descriptors.
///
/// You can stuff FDs in with the [`add`](#method.add) method, and
/// iterate over them one by one using the iterator structure returned
/// by [`iter`](#method.iter).
#[derive(Clone)]
pub struct Ring {
    read: RawFd,
    write: RawFd,

    /// The number of file descriptors contained in the ring buffer.
    pub count: u64,
}

impl fmt::Display for Ring {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#<Ring containing {} entries>", self.count)
    }
}

impl Drop for Ring {
    fn drop(&mut self) {
        // println!("Dropping sockets holding {} fds", self.count);
        unistd::close(self.write).unwrap();
        unistd::close(self.read).unwrap();
    }
}

/// Any sort of error that can occur while trying to speak the ring
/// buffer protocol
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum ProtocolError {
    /// Expected to receive an FD, but did not get one
    NoFDReceived(u64),

    /// Something approximating a Ring was sent over the socket, but the number format didn't parse
    RingFormatError,

    /// Expected one FD, got more
    TooManyFDsReceived,
}

#[derive(Debug)]
pub enum Error {
    /// A real error that prevents the Ring buffer from working
    Bad(nix::Error),

    /// An error that indicates some limit being reached. This is
    /// sometimes expected and realistic!
    Limit(nix::Error),

    /// A protocol error (e.g., messages on the socket didn't have the
    /// right format)
    Protocol(ProtocolError),
}

impl From<nix::Error> for Error {
    fn from(err: nix::Error) -> Error {
        match err {
            nix::Error::Sys(nix::errno::Errno::EMFILE) => Error::Limit(err),
            nix::Error::Sys(nix::errno::Errno::ETOOMANYREFS) => Error::Limit(err),
            nix::Error::Sys(nix::errno::Errno::EAGAIN) => Error::Limit(err),
            nix::Error::Sys(nix::errno::Errno::ENFILE) => Error::Limit(err),
            nix::Error::Sys(nix::errno::Errno::ENOSPC) => Error::Limit(err),
            nix::Error::Sys(_) => Error::Bad(err),
            nix::Error::InvalidPath => Error::Bad(err),
        }
    }
}

impl From<num::ParseIntError> for Error {
    fn from(_: num::ParseIntError) -> Error {
        Error::Protocol(ProtocolError::RingFormatError)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(_: str::Utf8Error) -> Error {
        Error::Protocol(ProtocolError::RingFormatError)
    }
}

/// A specialized Result type for fd Ring buffer operations.
pub type Result<T> = result::Result<T, Error>;

// Create a new Ring with a UNIX domain socket pair.
pub fn new() -> Result<Ring> {
    use super::unix_socket_pair;

    let (read, write) = try!(unix_socket_pair());
    // Adjust limits:
    let buf_size: usize = SEND_BUF_SIZE;
    try!(socket::setsockopt(write, socket::sockopt::SndBuf, &buf_size));
    return Ok(Ring {
        read: read,
        write: write,
        count: 0,
    });
}

#[derive(Clone)]
pub enum StashableThing<'a> {
    One(RawFd),
    Pair(&'a Ring),
}

#[derive(Clone)]
pub enum StashedThing {
    One(RawFd),
    Pair(Ring)
}

impl<'a> From<RawFd> for StashableThing<'a> {
    fn from(fd: RawFd) -> StashableThing<'a> {
        StashableThing::One(fd)
    }
}

impl<'a> From<&'a Ring> for StashableThing<'a> {
    fn from(ring: &'a Ring) -> StashableThing<'a> {
        StashableThing::Pair(ring)
    }
}

impl<'a> Ring {
    /// Adds an FD to a Ring, updating the count of contained FDs.
    /// Closing the FD to free up resources is left to the caller.
    ///
    /// # Errors
    /// * [`Bad(nix::Error)`](enum.Error.html#variant.Bad) - if any unforeseen condition occurs
    /// * [`Limit(nix::Error)`](enum.Error.html#variant.Limit) - if
    ///   the socket would block or any other limit runs over.
    pub fn add(&mut self, thing: &StashableThing) -> Result<()> {
        let n = try!(self.insert(thing));
        self.count += n;
        Ok(())
    }

    /// (internal) Add an FD to the ring, sending it down the `.write`
    /// end, and returns the number of entries made
    fn insert(&self, thing: &StashableThing) -> Result<u64> {
        let mut msg = String::from("");
        let mut fds: Vec<RawFd> = vec![];
        let mut buf: Vec<IoVec<&[u8]>> = vec![];
        match thing {
            &StashableThing::One(fd) => {
                msg.push('!');
                fds.push(fd);
            }
            &StashableThing::Pair(ring) => {
                msg.push_str(format!("{}", ring.count).as_str());
                fds.push(ring.read);
                fds.push(ring.write);
            }
        }
        buf.push(IoVec::from_slice(msg.as_bytes()));
        let cmsgs = vec![socket::ControlMessage::ScmRights(fds.as_slice())];
        try!(socket::sendmsg(self.write,
                             &buf.as_slice(),
                             cmsgs.as_slice(),
                             socket::MsgFlags::empty(),
                             None));
        Ok(1)
    }

    /// Removes and returns the head of the fd ring, updating count.
    pub fn pop(&mut self) -> Result<StashedThing> {
        let thing = try!(self.remove());
        self.count -= 1;
        Ok(thing)
    }

    /// (internal) Removes and returns the head of the ring from `.read`.
    fn remove(&self) -> Result<StashedThing> {
        // I assume we have no more than a 10^1023 FDs in there, but haha.
        let mut backing_buf: Vec<u8> = vec![0;1024];

        // TODO: deal with the constant 15 here.
        let mut cmsg: socket::CmsgSpace<([RawFd; 15])> = socket::CmsgSpace::new();
        let iov = IoVec::from_mut_slice(backing_buf.as_mut_slice());
        let mut iovs = vec![iov];
        let msg = try!(socket::recvmsg(self.read,
                                       &mut iovs.as_mut_slice(),
                                       Some(&mut cmsg),
                                       socket::MsgFlags::empty()));

        let read_buffer: &[u8] = iovs[0].as_slice();
        let read_bytes: &[u8] = &read_buffer[..msg.bytes];
        match msg.cmsgs().next() {
            Some(socket::ControlMessage::ScmRights(fds)) => {
                // TODO: this could probably handle the case of multiple FDs via buffers
                match fds.len() {
                    1 => {
                        let fd = fds[0];
                        let thing = StashedThing::One(fd);
                        Ok(thing)
                    }
                    2 => {
                        let count_str = try!(str::from_utf8(read_bytes));
                        let count: u64 = try!(u64::from_str(count_str));
                        let ring = Ring{
                            read: fds[0],
                            write: fds[1],
                            count: count
                        };
                        Ok(StashedThing::Pair(ring))
                    }
                    0 => Err(Error::Protocol(ProtocolError::NoFDReceived(1))),
                    _ => Err(Error::Protocol(ProtocolError::TooManyFDsReceived)),
                }
            }
            Some(_) => { panic!("Received something other than ScmRights! Wat."); }
            _ => Err(Error::Protocol(ProtocolError::NoFDReceived(2)))
        }
    }

    fn next(&self) -> Result<StashedThing> {
        let thing = try!(self.remove());
        match thing {
            StashedThing::One(fd) => {
                try!(self.insert(&StashableThing::from(fd)));
                Ok(StashedThing::One(fd))
            }
            StashedThing::Pair(ring) => {
                try!(self.insert(&StashableThing::from(&ring)));
                Ok(StashedThing::Pair(ring))
            }
        }
    }

    /// Returns an iterator on the FDs contained in the ring buffer
    pub fn iter(&self) -> RingIter {
        RingIter {
            ring: &self,
            offset: 0,
        }
    }
}

/// An iterator over the File descriptors contained in an FD ring buffer
pub struct RingIter<'a> {
    ring: &'a Ring,
    offset: u64,
}

impl<'a> Iterator for RingIter<'a> {
    type Item = StashedThing;

    fn next(&mut self) -> Option<StashedThing> {
        self.offset += 1;
        if self.offset > self.ring.count {
            return None;
        }
        match self.ring.next() {
            Ok(next_fd) => Some(next_fd),
            Err(e) => {
                panic!("Oops, {:?} happened at offset {} of {}", e, self.offset, self.ring.count);
            }
        }
    }
}

// Unit tests follow:

#[test]
fn it_can_create_a_ringbuffer() {
    let ring = new().unwrap();
    println!("Got a ring: {}", ring);
}

#[test]
fn adding_to_ring_works() {
    let mut ring = new().unwrap();
    let (one, two) = super::unix_socket_pair().unwrap();
    ring.add(&StashableThing::from(one)).unwrap();
    assert_eq!(1, ring.count);
    ring.add(&StashableThing::from(two)).unwrap();
    assert_eq!(2, ring.count);

    let other_ring = new().unwrap();
    ring.add(&StashableThing::from(&other_ring)).unwrap();
    assert_eq!(3, ring.count);

    let received = ring.pop().unwrap();
    match received {
        StashedThing::One(_) => {
            println!("Yay!");
        }
        _ => {
            panic!("Huh!");
        }
    }
}
