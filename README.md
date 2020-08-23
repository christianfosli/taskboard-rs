# To-Do App

A Kanban inspired to-do app for managing tasks and tracking progress.

The purpose of this application is mostly for me to play with web development
with rust, docker, kubernetes and elasticsearch.

Elasticsearch might be a weird choice to persist the to-do items.
I chose it simply to learn more about it.

## Development

### Prerequisites

Enable [BuildKit](https://docs.docker.com/develop/develop-images/build_enhancements/)
for docker and docker-compose, to reduce compile times:

```sh
. enable_buildkit.sh
```

### Run the code

```sh
docker-compose up -d --build
```
