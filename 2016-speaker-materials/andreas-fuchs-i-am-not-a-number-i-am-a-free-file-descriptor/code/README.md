# Programs that confuse the kernel with time traveling sockets

This Rust library (and associated tests) contains code that mis-uses
UNIX domain sockets to cause systems to run out of file descriptors
(globally), in a hilarious way.

If you want to run the code, read on! For background, see [the top-level README](../README.md#background--how-does-this-even-work).

## Running the code

You should try running this code! The scientific method demands that
you replicate my results! (-:

### Prerequisites

You'll need Rust 1.8, and a UNIX system. I tested on Linux and on OS
X, but I suppose FreeBSD and others will do just as well.

If you want to test this in the Linux configuration that I was
testing, you will also need Docker (I used
[dlite](https://github.com/nlf/dlite) on OS X, but see
[Compatibility](#compatibility--problems)).

### Building / Running

`make test` will build this repo and start tests on your currently-running system. If you

If you have Docker, you can run `make dockertest` to start tests in a
~standardized Linux environment.

`make testall` will run tests with the most verbose options activated
in both the local system and in Docker.

#### Build Options

* `TESTBT=1` will activate backtrace printing (this is less useful on
  OS X than you'd hope, sadly).

* `TESTOPT='--nocapture'` will print all output from tests, which will
  show you what the tests are actually up to.

Run `make` to see documentation of all the targets that make sense to run.


### Compatibility / Problems

Since we're playing with fire near system internals, there are
problems that can make the tests un-runnable. Here's a list of the
ones I've encountered:

* OS X has a ridiculously low system file limit. You'll probably want
  to run `sudo sysctl -w kern.maxfiles=20480`, otherwise you'll see
  the programs die early because the system file table runs over.

* The Linux kernel that ships with the Alpine distribution in the beta
  Docker.app returns bogus `ETOOMANYREFS` from perfectly innocent file
  descriptor operations. I believe this is a bug that popped up
  recently. At least dovecot
  [is affected by something similar](http://thread.gmane.org/gmane.mail.imap.dovecot/84985),
  also.
