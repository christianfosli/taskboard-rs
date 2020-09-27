# syntax = docker/dockerfile:1-experimental
FROM rust:1.46 AS builder
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
WORKDIR /usr/src/app
ARG API_URL=https://taskboard.cloud/api
ENV API_URL=$API_URL
ARG BUILD_PROFILE=release
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    wasm-pack build --${BUILD_PROFILE} --target web --out-name wasm --out-dir ./static

FROM nginx AS final
COPY --from=builder /usr/src/app/static /usr/share/nginx/html
COPY nginx-wasm-mime-type.conf /etc/nginx/conf.d/nginx-wasm-mime-type.conf
EXPOSE 80
