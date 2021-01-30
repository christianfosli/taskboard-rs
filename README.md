# Taskboard

![app](https://github.com/christianfosli/taskboard-rs/workflows/app/badge.svg)
![task-service](https://github.com/christianfosli/taskboard-rs/workflows/task-service/badge.svg)
![project-service](https://github.com/christianfosli/taskboard-rs/workflows/project-service/badge.svg)
![core-lib](https://github.com/christianfosli/taskboard-rs/workflows/core-lib/badge.svg)
[![core-lib](https://img.shields.io/crates/v/taskboard-core-lib)](https://crates.io/crates/taskboard-core-lib)
![infra](https://github.com/christianfosli/taskboard-rs/workflows/infra/badge.svg)

A Kanban inspired board for managing tasks/todo's and tracking progress.

Visit [https://www.taskboard.cloud](https://www.taskboard.cloud) to try it out!

This application is mostly a proof-of-concept of full-stack development with Rust.
It's also a nice chance for me to play with rust, docker, kubernetes and elasticsearch.

Elasticsearch might be a weird choice for persisting data.
I chose it simply to learn more about it.

## Development

### Prerequisites

* Docker >= v20.10 and preferably docker-compose

  * To use older versions of docker you'll need to add this to the top of the
    Dockerfiles

    ```Dockerfile
    # syntax = docker/dockerfile:1-experimental
    ```

* Enable [BuildKit](https://docs.docker.com/develop/develop-images/build_enhancements/)
  for docker and docker-compose.
  This is not strictly required, *but highly recommended*, because it allows
  caching builds so you don't have to build all the dependencies from scratch...

  ```sh
  . enable_buildkit.sh
  ```

### Run the code

```sh
docker-compose up -d --build
```

## Logs, Metrics and Monitoring

Basic health info is available at
[https://www.taskboard.cloud/healthz](https://taskboard.cloud/healthz).
More details metrics are password protected. LMK if you need access.
