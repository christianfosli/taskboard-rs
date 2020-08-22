FROM rust:1.45 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as final
RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/todo-api /usr/local/bin/todo-api
EXPOSE 8000
CMD ["todo-api"]
