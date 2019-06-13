FROM clux/muslrust:nightly

WORKDIR /build

RUN USER=root cargo new --bin base
COPY ./base/Cargo.toml ./base/Cargo.toml

RUN USER=root cargo new --bin bounded-queue
COPY ./bounded-queue/Cargo.toml ./bounded-queue/Cargo.toml

RUN USER=root cargo new --bin ctf
COPY ./ctf/Cargo.toml ./ctf/Cargo.toml

RUN USER=root cargo new --bin ffa
COPY ./ffa/Cargo.toml ./ffa/Cargo.toml

RUN USER=root cargo new --bin server
COPY ./server/Cargo.toml ./server/Cargo.toml

RUN USER=root cargo new --bin special-map
COPY ./special-map/Cargo.toml ./special-map/Cargo.toml

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release

RUN rm ./base/src/*.rs
RUN rm ./bounded-queue/src/*.rs
RUN rm ./ctf/src/*.rs
RUN rm ./ffa/src/*.rs
RUN rm ./server/src/*.rs
RUN rm ./special-map/src/*.rs

COPY . /build

RUN cargo build --release
RUN mkdir /artifacts
RUN mv target/x86_64-unknown-linux-musl/release/airmash-server-base /artifacts/airmash-server

FROM alpine:latest

EXPOSE 3501

COPY --from=0 /artifacts/airmash-server /app/airmash-server

ENV RUST_LOG=info,ws=warn

ENTRYPOINT [ "/app/airmash-server" ]
