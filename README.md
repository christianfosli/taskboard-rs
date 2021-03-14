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

## Architecture Overview

Here's a high level overview of the exposed services:

![Architecture overview of taskboard.cloud](architecture-overview.svg)

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

### Potential issues

Your local elasticsearch cluster becomes read-only if you're low on disk space.
This makes creating tasks and projects fail.
The elasticsearch container will emit logs similar to this.

```json
{
  "type": "server",
  "level": "WARN",
  "component": "o.e.c.r.a.DiskThresholdMonitor",
  "cluster.name": "docker-cluster",
  "message": "flood stage disk watermark [95%] exceeded on [OZOgTVsrTlerKqoChHnhYw][3d7d5a8abd03][/usr/share/elasticsearch/data/nodes/0] free: 2.1gb[3%], all indices on this node will be marked read-only"
}
```

In my experience it was caused by docker images and build cache filling up my
root partition. These commands should help free up some space:

```console
docker system df           # check how much space docker is taking
docker image prune -a      # remove all unused images
docker build prune         # remove dangling build cache
```

## Logs, Metrics and Monitoring

Basic health info is available at
[https://www.taskboard.cloud/healthz](https://www.taskboard.cloud/healthz).
More detailed metrics are password protected. LMK if you need access.
