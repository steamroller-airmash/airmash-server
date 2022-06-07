FROM rust:1.61.0-slim-bullseye as build-env

RUN apt-get update \
  && apt-get install -y dwz \
  && apt-get clean

WORKDIR /build
COPY . /build

ARG TARGET

RUN cargo build --profile prod --bin airmash-server-${TARGET}
RUN mv target/prod/airmash-server-${TARGET} target/airmash-server
RUN dwz -L none -l none --odr target/airmash-server

FROM debian:bullseye-slim

COPY --from=build-env /build/target/airmash-server /

EXPOSE 3501/tcp
ENV RUST_LOG=info

ENTRYPOINT [ "/airmash-server" ]
