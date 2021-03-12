FROM netflix-dev

RUN mkdir -p /usr/src/netflix

WORKDIR /usr/src/netflix

COPY . /usr/src/netflix

RUN apt-get update -y
RUN apt-get install -y lld

RUN cargo install cargo-watch
