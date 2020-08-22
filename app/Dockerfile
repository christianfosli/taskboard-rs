FROM rust:1.45 as builder
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
WORKDIR /usr/src/app
COPY . .
RUN wasm-pack build --target web --out-name wasm --out-dir ./static

FROM nginx as final
COPY --from=builder /usr/src/app/static /usr/share/nginx/html
