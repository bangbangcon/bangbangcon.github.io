extern crate filedes;
extern crate nix;

use filedes::ring;
use filedes::{add_two_sockets_to_ring,add_tmpfile_to_ring};
use std::process::Command;
use std::io;
use std::io::Write;

/// Linux gets *really* *really* slow at a certain point (if you put
/// UNIX domain sockets on the ring buffers). If you should want to
/// play with `add_two_sockets_to_ring`, you should bump this limit
/// down to, say, 50.
const ARBITRARY_LIMIT: u64 = 400;

// In Linux, this works! We can send rings down rings, and the system
// will get very very slow, but sockets containing FDs can be sent
// down sockets, and can be read off them.
//
// In OS X, I'm getting NoFDReceived messages, which indicates to me
// that the OS is closing those. Ugh!
#[cfg(not(target_os="macos"))]
#[test]
fn adding_rings_to_rings_works() {
    let mut outer_ring = ring::new().unwrap();
    let mut total = 0;
    let mut outer_entries = 0;
    println!("One dot corresponds to one entry on the outer ring:");

    'outer: loop {
        let mut inner_ring = ring::new().unwrap();
        'inner: loop {
            match add_tmpfile_to_ring(&mut inner_ring) {
                Ok(n) => { total += n; }
                Err(ring::Error::Limit(nix::Error::Sys(nix::errno::Errno::EAGAIN))) => {
                    if inner_ring.count > 1 {
                        print!(".");
                        io::stdout().flush().unwrap();
                        match outer_ring.add(&ring::StashableThing::from(&inner_ring)) {
                            Ok(()) => {}
                            Err(_) => {
                                // We have to throw away this inner ring, adjust totals for it:
                                total -= inner_ring.count;
                                break 'outer
                            }
                        }
                        break 'inner;
                    } else {
                        break 'outer;
                    }
                }
                Err(ring::Error::Limit(e)) => {
                    println!("\nI hit {} - this means something global is full, probably", e);
                    total -= inner_ring.count;
                    break 'outer;
                }
                e => { panic!("\nError {:?}", e); }
            }
        }
        outer_entries += 1;
        if outer_entries > ARBITRARY_LIMIT {
            break;
        }
    }
    println!("Assembled an outer ring of {} for a total of {} FDs, lsof output follows:", outer_ring, total);
    let output = Command::new("lsof")
        .arg("-p")
        .arg(format!("{}", nix::unistd::getpid()))
        .output().unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
    assert!(outer_ring.count > 1);

    println!("Now I'm going to close all these one by one, hang tight.");
    let mut closed = 0;
    for inner_thing in outer_ring.iter() {
        match inner_thing {
            ring::StashedThing::Pair(mut inner_ring) => {
                while inner_ring.count > 0 {
                    let thing = inner_ring.pop().unwrap();
                    match thing {
                        ring::StashedThing::Pair(_) => {
                            panic!("I don't know how I could get to a ring in inner");
                        }
                        ring::StashedThing::One(fd) => {
                            nix::unistd::close(fd).unwrap();
                            closed += 1;
                        }
                    }
                }
            }
            _ => { panic!("Don't know how I could get to a non-ring in outer"); }
        }
    }
    assert_eq!(total, closed);

}
