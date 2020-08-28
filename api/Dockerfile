# syntax = docker/dockerfile:1-experimental
FROM rust:1.46 as builder
WORKDIR /usr/src/app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo install --path .

FROM debian:buster-slim as final
RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/taskboard-api /usr/local/bin/taskboard-api
EXPOSE 8000
CMD ["taskboard-api"]
