extern crate filedes;
extern crate nix;

use std::thread;
use nix::sys::socket;
use nix::sys::uio::IoVec;
use std::os::unix::io::RawFd;
use nix::unistd;

#[test]
fn it_sends_fds() {
    filedes::setup().unwrap();
    let s_sock = filedes::server_socket("mysock3").unwrap();
    thread::spawn(move || {
        let conn = socket::accept(s_sock).unwrap();

        let buf = vec![IoVec::from_slice("!".as_bytes())];

        let fds = vec![conn];
        let cmsgs = vec![socket::ControlMessage::ScmRights(fds.as_slice())];
        socket::sendmsg(conn,
                        &buf.as_slice(),
                        cmsgs.as_slice(),
                        socket::MsgFlags::empty(),
                        None)
            .unwrap();
        unistd::close(s_sock).unwrap();
    });
    let sock = filedes::connect_to_socket("mysock3").unwrap();

    let mut backing_buf = vec![0];
    let mut buf = vec![IoVec::from_mut_slice(&mut backing_buf)];
    let mut cmsg: socket::CmsgSpace<([RawFd; 15])> = socket::CmsgSpace::new();
    let msg = socket::recvmsg(sock,
                              &mut buf.as_mut_slice(),
                              Some(&mut cmsg),
                              socket::MsgFlags::empty())
        .unwrap();
    assert_eq!(1, msg.cmsgs().count());
    filedes::teardown().unwrap();
}
