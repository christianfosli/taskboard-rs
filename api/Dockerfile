# syntax = docker/dockerfile:1-experimental
FROM rust:1.46 as builder
WORKDIR /usr/src/app
ARG CARGO_INSTALL_OPTIONS
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo test && cargo install ${CARGO_INSTALL_OPTIONS} --path .

FROM debian:buster-slim as final
RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/taskboard-api /usr/local/bin/taskboard-api
ENV RUST_LOG=info
EXPOSE 8000
CMD ["taskboard-api"]
