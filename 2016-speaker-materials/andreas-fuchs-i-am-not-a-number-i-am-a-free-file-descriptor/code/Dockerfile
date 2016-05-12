FROM ubuntu:14.04
MAINTAINER Andreas Fuchs <asf@boinkor.net>

RUN apt-get update && apt-get install -y curl lsof strace screen
RUN curl https://static.rust-lang.org/dist/rust-1.8.0-x86_64-unknown-linux-gnu.tar.gz | tar zxf - -C /opt && /opt/rust-1.8.0-x86_64-unknown-linux-gnu/install.sh
RUN apt-get install -y build-essential

RUN env USER=root cargo new /tmp/cache
ADD Cargo.toml /tmp/cache
ADD Cargo.lock /tmp/cache
RUN cd /tmp/cache ; cargo build

ADD . ./src
RUN cp -Rp /tmp/cache/target /src

CMD cd /src ; cargo test
