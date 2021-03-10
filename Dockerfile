FROM rust

RUN mkdir -p /usr/src/netflix

WORKDIR /usr/src/netflix

COPY . /usr/src/netflix

RUN cargo install cargo-watch
