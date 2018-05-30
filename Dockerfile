FROM clux/muslrust:nightly

WORKDIR /build

ADD Cargo.toml Cargo.lock /build/
RUN mkdir src
RUN echo > src/main.rs
RUN cargo fetch
RUN rm -rf ./*

ADD . /build/

RUN cargo build --release
RUN mv target/release/airmash-server /artifacts/airmash-server

FROM alpine:latest

RUN apk add --no-cache supervisor

WORKDIR /app

ADD supervisor.conf /app/supervisor.conf
COPY --from=0 /artifacts/airmash-server /app/airmash-server

ENTRYPOINT supervisord
