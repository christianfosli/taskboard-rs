FROM rust:1.59 as builder
WORKDIR /usr/src/app
ARG CARGO_INSTALL_OPTIONS
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo test && cargo install ${CARGO_INSTALL_OPTIONS} --path .

FROM debian:bullseye-slim as final
RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/task-service /usr/local/bin/task-service
ENV RUST_LOG=info
EXPOSE 80
CMD ["task-service"]
