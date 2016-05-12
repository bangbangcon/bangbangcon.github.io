extern crate filedes;
extern crate nix;

use filedes::ring;
use filedes::{add_two_sockets_to_ring,add_tmpfile_to_ring};
use std::os::unix::io::RawFd;
use std::process::Command;

#[test]
fn adding_many_to_a_ring_works() {
    let mut ring = ring::new().unwrap();

    loop {
        match add_tmpfile_to_ring(&mut ring) {
            Ok(_) => {}
            Err(ring::Error::Limit(e)) => {
                println!("I hit {}", e);
                break;
            }
            Err(e) => { panic!("Oops, {:?}", e); }
        }
    }
    let mut additional_fds: Vec<RawFd> = vec!();
    loop {
        match filedes::unix_socket_pair() {
            Ok((one, two)) => {
                additional_fds.push(one);
                additional_fds.push(two);
            }
            Err(e) => {
                println!("Hit {}, aborting", e);
                break;
            }
        }

    }
    println!("I managed to store a bunch of FDs in {}", ring);
    println!("...and I opened {} FDs", additional_fds.len());
    assert!(additional_fds.len() > 0);

    // println!("Waiting 60s");
    // std::thread::sleep(std::time::Duration::from_secs(60));

    println!("Closing the additional FDs now...");
    for fd in additional_fds {
        nix::unistd::close(fd).unwrap();
    }
    println!("I still have {} FDs open, but let's see! lsof output follows:", ring.count);
    let output = Command::new("lsof")
        .arg("-p")
        .arg(format!("{}", nix::unistd::getpid()))
        .output().unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());

    let should_close = ring.count;
    let mut closed = 0;
    println!("Closing the stashed FDs now...");
    for thing in ring.iter() {
        closed += 1;
        match thing {
            ring::StashedThing::One(fd) => {
                nix::unistd::close(fd).unwrap();
            }
            ring::StashedThing::Pair(_) => {}
        }
    }
    assert_eq!(should_close, closed);

    println!("Closing the stashed FDs a second time, properly...");
    while ring.count > 0 {
        let thing = ring.pop().unwrap();
        match thing {
            ring::StashedThing::One(fd) => {
                nix::unistd::close(fd).unwrap();
            }
            ring::StashedThing::Pair(_) => {
            }
        }
    }
}
