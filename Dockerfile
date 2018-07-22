FROM clux/muslrust:nightly

WORKDIR /build/server

# Cache downloaded packages to avoid redownloading
# all dependencies every time a project file is 
# changed on the server. Since this project downloads
# a large number of dependencies, this should save 
# a decent amount of bandwith
ADD server/Cargo.toml /build/server/
RUN mkdir src

# Need to add specgen so cargo fetch works
COPY specgen /build/specgen
COPY bounded-queue /build/bounded-queue
COPY special-map /build/special-map

# Fetch all dependencies to save bandwith
RUN echo > src/main.rs
RUN cargo fetch
RUN rm -rf src

ADD server /build/server
ADD base /build/base

WORKDIR /build/base

RUN cargo build --release
RUN mkdir /artifacts
RUN mv target/x86_64-unknown-linux-musl/release/airmash-server-base /artifacts/airmash-server

FROM alpine:latest

COPY --from=0 /artifacts/airmash-server /app/airmash-server

ENTRYPOINT [ "/app/airmash-server" ]
