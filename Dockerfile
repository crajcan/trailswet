FROM rust

RUN mkdir -p /usr/src/netflix

WORKDIR /usr/src/netflix

COPY . /usr/src/netflix

RUN echo DATABASE_URL="postgres://netflix:password@host.docker.internal/netflix" > .env
RUN echo SOCKET_ADDRESS="host.docker.internal:3000" >> .env

RUN cargo install cargo-watch
