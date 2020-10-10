# Taskboard

![app](https://github.com/christianfosli/taskboard-rs/workflows/app/badge.svg)
![task-service](https://github.com/christianfosli/taskboard-rs/workflows/task-service/badge.svg)
![project-service](https://github.com/christianfosli/taskboard-rs/workflows/project-service/badge.svg)
![core-lib](https://github.com/christianfosli/taskboard-rs/workflows/core-lib/badge.svg)
[![core-lib](https://img.shields.io/crates/v/taskboard-core-lib)](https://crates.io/crates/taskboard-core-lib)
![infra](https://github.com/christianfosli/taskboard-rs/workflows/infra/badge.svg)

A Kanban inspired board for managing tasks/todo's and tracking progress.

This application is mostly a proof-of-concept of full-stack development with
Rust.
It's also a nice chance for me to play with rust, docker, kubernetes and
elasticsearch.

Elasticsearch might be a weird choice for persisting data.
I chose it simply to learn more about it.

## Development

### Prerequisites

* Docker and preferably docker-compose

* Enable [BuildKit](https://docs.docker.com/develop/develop-images/build_enhancements/)
  for docker and docker-compose.
  This is required because the Dockerfiles are using buildkit features for caching rust compilations.

  ```sh
  . enable_buildkit.sh
  ```

### Run the code

```sh
docker-compose up -d --build
```
