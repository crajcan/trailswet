FROM rust

RUN mkdir -p /usr/src/netflix

WORKDIR /usr/src/netflix

COPY . /usr/src/netflix

RUN echo DATABASE_URL="postgres://netflix:password@host.docker.internal/netflix" > .env
RUN echo SOCKET_ADDRESS="0.0.0.0:80" >> .env

RUN cargo install cargo-watch
