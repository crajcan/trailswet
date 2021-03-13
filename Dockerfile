FROM rust:1.49-buster

RUN mkdir -p /usr/src/netflix

WORKDIR /usr/src/netflix

COPY . /usr/src/netflix

RUN apt-get update -y
RUN apt-get install -y lld

RUN cargo install cargo-watch

ENV DATABASE_URL=postgres://netflix:password@host.docker.internal/netflix
ENV SOCKET_ADDRESS=0.0.0.0:80

RUN cargo build
