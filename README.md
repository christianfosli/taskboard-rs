# Taskboard

---

## :warning: Obsolete :warning:

I used this project to learn about Rust and Kubernetes,
but I have (per 2023) not been updating it in a while,
so many of the tools are now out of date or no longer best-practice.
I have decomissioned the related cloud resources for running this page (meaning the URL's no longer work)
and archived the git repository for now.

---

![app](https://github.com/christianfosli/taskboard-rs/workflows/app/badge.svg)
![task-service](https://github.com/christianfosli/taskboard-rs/workflows/task-service/badge.svg)
![project-service](https://github.com/christianfosli/taskboard-rs/workflows/project-service/badge.svg)
![core-lib](https://github.com/christianfosli/taskboard-rs/workflows/core-lib/badge.svg)
![cargo audit](https://github.com/christianfosli/taskboard-rs/actions/workflows/cargo_audit.yaml/badge.svg)
[![core-lib](https://img.shields.io/crates/v/taskboard-core-lib)](https://crates.io/crates/taskboard-core-lib)
![infra](https://github.com/christianfosli/taskboard-rs/workflows/infra/badge.svg)

A Kanban inspired board for managing tasks/todo's and tracking progress.

Visit [https://www.taskboard.cloud](https://www.taskboard.cloud) to try it out!

This application is mostly a proof-of-concept of full-stack development with Rust.
It's also my personal playground for testing out web development with rust,
and kubernetes, and elasticsearch.

Elasticsearch might be a weird choice for persisting data. I chose it to learn more about it.

## Architecture Overview

Here's a high level overview of the different exposed services:

```mermaid
graph TD
    subgraph kubernetes
    I(ingress-nginx) ---> APP(taskboard app<br/>www.taskboard.cloud)
    I ---> P(project service<br/>api.taskboard.cloud/project)
    I ---> T(task service<br/>api.taskboard.cloud/task)
    I ----> M(linkerd dashboard<br/>metrics.taskboard.cloud)
    I ----> K(kibana<br/>logs.taskboard.cloud)
end
```

Elasticsearch is used for persisting application data and logs.

## Metrics, Logs and Monitoring

Basic health info is available at
[www.taskboard.cloud/healthz](https://www.taskboard.cloud/healthz).

More detailed metrics are available on [metrics.taskboard.cloud](https://metrics.taskboard.cloud),
and logs are available through kibana on [logs.taskboard.cloud](https://logs.taskboard.cloud).
These are password protected. Let me know if you need would like access.

Note that I'm running all of this on a **single-node cheap-ish AKS cluster**.
Thus there might be some unreliability, especially when I try out something new.

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
  This is not required, *but recommended* because it significantly speeds up
  subsequent builds due to caching.

  ```sh
  export DOCKER_BUILDKIT=1
  export COMPOSE_DOCKER_CLI_BUILD=1
  ```

### Run the code

```sh
docker-compose up -d --build
```

### Likely issues when running locally

#### Port 80 not available

docker-compose.yaml maps the nginx container serving the front-end app to
http://localhost

```yaml
ports:
  - "80:80"
```

This will fail if you already have something running on port 80.
Feel free to change it to something else.

#### Front-end app not updating

It's probably cached in the browser. Disable cache or hard-reload
(control-shift-r in chrome).
The problem is mitigated for CI builds by appending the (short) commit hash to
the file names.

#### Creating projects and tasks fails when low disk-space available

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
docker builder prune       # remove dangling build cache
```
