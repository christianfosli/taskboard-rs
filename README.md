# Taskboard

![taskboard-app](https://github.com/christianfosli/taskboard-rs/workflows/taskboard-app/badge.svg)
![taskboard-api](https://github.com/christianfosli/taskboard-rs/workflows/taskboard-api/badge.svg)
![taskboard-core-lib](https://github.com/christianfosli/taskboard-rs/workflows/taskboard-core-lib/badge.svg)
[![taskboard-core-lib](https://img.shields.io/crates/v/taskboard-core-lib)](https://crates.io/crates/taskboard-core-lib)
![taskboard-infra](https://github.com/christianfosli/taskboard-rs/workflows/taskboard-infra/badge.svg)

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
