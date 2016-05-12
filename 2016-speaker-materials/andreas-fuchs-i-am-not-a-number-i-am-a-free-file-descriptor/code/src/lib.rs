#![allow(dead_code)]
#![deny(warnings)]
#![crate_type = "lib"]

//! The top-level module filedes contains convenience / test stuff for playing with file descriptors.
//!
//! The most interesting one is probably [`unix_socket_pair`](fn.unix_socket_pair.html).


extern crate nix;
extern crate libc;

pub mod ring;

use nix::sys::socket;
use nix::NixPath;
use std::ffi::{CString,OsStr};
use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::os::unix::io::RawFd;
use std::os::unix::ffi::OsStrExt;

const BASE_PATH: &'static str = "/tmp/filedes_fun/";
const MAX_BACKLOG_QUEUE: usize = 265;

const SOCKET_TYPE: socket::SockType = socket::SockType::Stream;
const SOCKET_PROTO: nix::c_int = 0;

// Setup the directory they'll live in.
pub fn setup() -> io::Result<()> {
    fs::create_dir_all(Path::new(self::BASE_PATH))
}

pub fn teardown() -> io::Result<()> {
    fs::remove_dir_all(Path::new(self::BASE_PATH))
}

fn sockpath(path: &str) -> PathBuf {
    Path::new(BASE_PATH).join(Path::new(path))
}

pub fn make_socket_addr(path: &str) -> Result<socket::SockAddr, nix::Error> {
    socket::SockAddr::new_unix(sockpath(path).as_path())
}

pub fn server_socket(path: &str) -> Result<RawFd, nix::Error> {
    let socket = try!(socket::socket(socket::AddressFamily::Unix,
                                     SOCKET_TYPE,
                                     socket::SockFlag::empty(),
                                     SOCKET_PROTO));
    let sockaddr = try!(make_socket_addr(path));
    try!(socket::bind(socket, &sockaddr));
    try!(socket::listen(socket, MAX_BACKLOG_QUEUE));
    Ok(socket)
}

pub fn connect_to_socket(path: &str) -> Result<RawFd, nix::Error> {
    let socket = try!(socket::socket(socket::AddressFamily::Unix,
                                     SOCKET_TYPE,
                                     socket::SockFlag::empty(),
                                     SOCKET_PROTO));
    let sockaddr = try!(make_socket_addr(path));
    try!(socket::connect(socket, &sockaddr));
    Ok(socket)
}

/// Creates a socketpair in the UNIX domain and returns it.
pub fn unix_socket_pair() -> Result<(RawFd, RawFd), nix::Error> {
    return socket::socketpair(socket::AddressFamily::Unix,
                              SOCKET_TYPE,
                              SOCKET_PROTO,
                              socket::SOCK_NONBLOCK);
}

/// Creates a new pair of sockets with {unix_socket_pair} and adds it
/// to the ring, and returns the number of sockets added.  This is the
/// easiest way to get throwaway file descriptor outside wrapping
/// `libc::mkstemp` (see [`add_tmpfile_to_ring`](fn.add_tmpfile_to_ring.html) for that) (:
///
/// This tries to close all file descriptors properly, but probably
/// fails & leaks them at various points.
pub fn add_two_sockets_to_ring(ring: &mut ring::Ring) -> ring::Result<u64> {
    let (one, two) = try!(unix_socket_pair());
    match ring.add(&ring::StashableThing::from(one)) {
        Ok(()) => {
            try!(nix::unistd::close(one));
        }
        Err(ring::Error::Limit(e)) => {
            println!("I hit {}", e);
            try!(nix::unistd::close(one));
            try!(nix::unistd::close(two));
            return Err(ring::Error::Limit(e));
        }
        Err(e) => {
            return Err(e);
        }
    }
    match ring.add(&ring::StashableThing::from(two)) {
        Ok(()) => {
            try!(nix::unistd::close(two));
            Ok(2)
        },
        Err(ring::Error::Limit(e)) => {
            println!("I hit {}", e);
            try!(nix::unistd::close(two));
            return Ok(1);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

/// Returns the FD of an unlinked temporary file.
#[inline]
fn mkstemp<P: ?Sized + NixPath>(template: &P) -> ring::Result<(RawFd, PathBuf)> {
    let (fd, pathname) = try!(template.with_nix_path(|path| {
        let owned_path = path.to_owned();
        let path_ptr = owned_path.into_raw();
        unsafe {
            (libc::mkstemp(path_ptr), CString::from_raw(path_ptr))
        }
    }));
    try!(nix::Errno::result(fd));
    Ok((fd, Path::new(OsStr::from_bytes(pathname.as_bytes())).to_owned()))
}

#[cfg(not(target_os="linux"))]
fn throwaway_file() -> ring::Result<RawFd> {
    let (fd, name) = try!(mkstemp("/tmp/filedes_fun.XXXXXXXXXXXX"));
    nix::unistd::unlink(name.as_path()).unwrap();
    Ok(fd)
}

#[cfg(target_os="linux")]
fn throwaway_file() -> ring::Result<RawFd> {
    let name = CString::new("foo").unwrap();
    let fd = try!(nix::sys::memfd::memfd_create(name.as_ref(), nix::sys::memfd::MemFdCreateFlag::empty()));
    Ok(fd)
}

/// Uses `mkstemp` to create an open, unlinked temporary file and add
/// its file descriptor to the [`Ring`](ring/struct.Ring.html)
/// structure.
///
/// This function closes the temporary file descriptor in any case
/// (successs or error).
pub fn add_tmpfile_to_ring(ring: &mut ring::Ring) -> ring::Result<u64> {
    let fd = try!(throwaway_file());
    match ring.add(&ring::StashableThing::from(fd)) {
        Ok(()) => {
            try!(nix::unistd::close(fd));
            Ok(1)
        }
        Err(ring::Error::Limit(e)) => {
            try!(nix::unistd::close(fd));
            return Err(ring::Error::Limit(e));
        }
        Err(e) => {
            println!("I don't understand what {:?} is", e);
            try!(nix::unistd::close(fd));
            return Err(e);
        }
    }
}
