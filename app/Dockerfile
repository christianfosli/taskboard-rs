FROM rust:1.52 AS builder
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
WORKDIR /usr/src/app
ARG VERSION=latest
ARG TASK_SERVICE_URL=https://api.taskboard.cloud/task
ENV TASK_SERVICE_URL=$TASK_SERVICE_URL
ARG PROJECT_SERVICE_URL=https://api.taskboard.cloud/project
ENV PROJECT_SERVICE_URL=$PROJECT_SERVICE_URL
ARG BUILD_PROFILE=release
COPY . .
RUN sed -i 's/$VERSION/'"$VERSION"'/g' ./static/index.html \
    && mv ./static/style.css ./static/style-${VERSION}.css
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    wasm-pack build --${BUILD_PROFILE} --target web --out-name wasm-${VERSION} --out-dir ./static

FROM nginx AS final
COPY ./nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /usr/src/app/static /usr/share/nginx/html
EXPOSE 80
