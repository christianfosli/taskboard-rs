# Taskboard

![app](https://github.com/christianfosli/taskboard-rs/workflows/app/badge.svg)
![task-service](https://github.com/christianfosli/taskboard-rs/workflows/task-service/badge.svg)
![project-service](https://github.com/christianfosli/taskboard-rs/workflows/project-service/badge.svg)
![core-lib](https://github.com/christianfosli/taskboard-rs/workflows/core-lib/badge.svg)
[![core-lib](https://img.shields.io/crates/v/taskboard-core-lib)](https://crates.io/crates/taskboard-core-lib)
![infra](https://github.com/christianfosli/taskboard-rs/workflows/infra/badge.svg)

A Kanban inspired board for managing tasks/todo's and tracking progress.

The purpose of this application is mostly for me to play with web development
with rust, docker, kubernetes and elasticsearch.

Elasticsearch might be a weird choice to persist the to-do items.
I chose it simply to learn more about it.

## Development

### Prerequisites

* Docker and preferably docker-compose

* Enable [BuildKit](https://docs.docker.com/develop/develop-images/build_enhancements/)
  for docker and docker-compose, to reduce compile times:

  ```sh
  . enable_buildkit.sh
  ```

### Run the code

```sh
docker-compose up -d --build
```
