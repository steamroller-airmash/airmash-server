FROM clux/muslrust:nightly

WORKDIR /build

# Cache downloaded packages to avoid redownloading
# all dependencies every time a project file is 
# changed on the server. Since this project downloads
# a large number of dependencies, this should save 
# a decent amount of bandwith
ADD Cargo.toml Cargo.lock /build/
RUN mkdir src
# Fetch all dependencies to save bandwith
RUN echo > src/main.rs
RUN cargo fetch
RUN rm -rf src

ADD build.rs /build
ADD src /build/src

RUN cargo build --release
RUN mkdir /artifacts
RUN mv target/x86_64-unknown-linux-musl/release/airmash-server /artifacts/airmash-server

FROM alpine:latest

RUN apk add --no-cache supervisor

WORKDIR /app

ADD supervisor.conf /app/supervisor.conf
COPY --from=0 /artifacts/airmash-server /app/airmash-server

ENTRYPOINT supervisord -c /app/supervisor.conf
