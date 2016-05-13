# I'm not a number, I'm a free file descriptor!!1 (our protagonist promptly disappears down a wormhole)

Welcome to the research speaker materials for my talk at !!con 2016!

You can find a PDF presentation in the [presentation/ directory]()

You can find the canonical git repo (that contains all of what's in
here, plus more links) at
[https://github.com/antifuchs/bangbangcon-talk-2016](https://github.com/antifuchs/bangbangcon-talk-2016).

## Background / how does this even work

First, two terms: "file descriptor" is a number referring to an entry
in the process's file table. Those entries point to global "file
descript*ion*" in the kernel. If you're confused now, these are the
terms that POSIX uses. I'm very sorry.

Now that the technical details are out of the way, here's what is
happening: My colleague [Nelson](https://nelhage.com) once told me
over lunch how UNIX domain sockets are super weird in Linux: You can
very easily write a program that runs the Kernel out of file
descriptions and even run the kernel out of memory!

This intrigued me, so I decided to write up some test programs to do
exactly this.

The way this works is that the tests here create many throwaway file
descriptors (on the Linux, this uses `memfd_create`; everywhere else,
it just unlinks `mkstemp`ed files), and then send them into a UNIX
domain socket pair, closing the files afterwards.

On Linux, this lets you get 500 file descriptors past the per-process
file limit, or more (depending on how many FDs you send in a single
control message; I opted to send a single one because that made the
type signatures easier).

Of course, that's not enough! We want pathological behavior! No, we
DEMAND it! OK, fine. Here's what you do then: You take the ends of
this UNIX domain socket pair (the "inner ring"), and then send them
down another UNIX domain socket pair (the "outer ring"). Then you
close that old inner ring, make a new one and start from the top,
until the kernel runs out of file descriptions entries in its file
table.

### What is happening?

Wheeeee! This works on Linux, because it has a
[socket GC](https://github.com/torvalds/linux/blob/v4.3/net/unix/garbage.c)
(do check out that file, the change log at the top is *amazing*). When
you close a pair of UNIX domain sockets, Linux recursively traverses
UNIX domain sockets from open roots, marking all reachable FDs as open
and then closing the ones it can't reach.

On Mac OS X (and other BSDs, apparently), this trick doesn't work,
because as soon as you close a pair of sockets, the FDs contained in
the socket pair are closed. This is eminently reasonable, but ever so
slightly less fun than the machinery Linux has.

### Further work: Can you break more things?

You can run the Linux kernel out of memory! My test programs make one
new temp FD for each message being sent, until the global file table
runs over. But you can also create *a single file throwaway
descriptor* and then use
[`dup`](http://man7.org/linux/man-pages/man2/dup.2.html) to create
copies of that single global file description entry! You'll be able to
store many more messages in rings (and will probably have to recurse
ring storage a bit), but at some point the kernel itself might run out
of memory. Wheeeeeeeeeeeeeee!

Another super fun thing to do is use UNIX domain sockets for those
temporary FDs: I use `memfd`s because they don't trigger recursive
garbage collection slowdown *as much* as if you used UNIX sockets. You
can slow everything down by *a lot* with just a few levels of UNIX
domain sockets stored in each other. I didn't do the math but I
believe this is
[accidentally (or I guess we're doing it intentionally) quadratic](http://accidentallyquadratic.tumblr.com/).

## The code

The source code for my research is in the
[code](code) directory.

I wrote these test programs in Rust (1.8.0), using
[nix](https://github.com/nix-rust/nix) for the UNIX interactions. The
programs are all done as integration-style tests in
[the tests directory](code/tests/).

You can [browse the source](code/src) (if anything isn't clear, I'm very sorry!
I'm pretty new to Rust, and I coded under a bit of time pressure. I'll
try to improve it if you
[file an issue](https://github.com/antifuchs/bangbangcon-talk-2016/issues/new)!)

There exist some
[API docs for the ring buffer](https://antifuchs.github.io/bangbangcon-talk-2016/file-descriptor-fun.rustdoc/filedes/ring/index.html).

See the [README.md in code/](code/README.md) for details
on how to run / build this.

## All the thanks

* [Nelson](https://nelhage.com/) for the initial inspiration,
  continual guidance and handholding through the various problems I
  encountered.

* [Julia](http://jvns.ca/), for the feedback that made this
  presentation way more fun than it was initially.

* [Kamal](http://kamalmarhubi.com/) for this post on
  [nix](http://kamalmarhubi.com/blog/2016/04/13/rust-nix-easier-unix-systems-programming-3/)
  that motivated me to write the research programs for this talk in
  Rust - I would probably never have finished writing this in C with
  all my hair still attached to my head.

* Last but definitely not least, [Veronika](http://bruin.at) for
  always encouraging me <3
