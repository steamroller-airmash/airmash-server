FROM clux/muslrust:nightly

WORKDIR /build
COPY . /build

RUN cargo build --release
RUN mkdir /artifacts
RUN mv target/x86_64-unknown-linux-musl/release/airmash-server-ffa /artifacts/airmash-server

FROM alpine:latest

EXPOSE 3501

COPY --from=0 /artifacts/airmash-server /app/airmash-server

ENTRYPOINT [ "/app/airmash-server" ]
