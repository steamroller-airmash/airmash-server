FROM rust:latest

RUN apt-get update \
	&& apt-get install -y openssl libssl-dev \
	&& rustup default nightly

WORKDIR /app

RUN USER=root cargo new dummy --bin
COPY Cargo.toml Cargo.lock /app/dummy/
RUN cd dummy \
	&& cargo build --release \
	&& cd /app \
	&& rm -rf /dummy

ADD . /app

RUN cargo install --path . \
	&& rm -rf /app/* \
	&& rm -rf ~/.cargo

ENTRYPOINT airmash-server
