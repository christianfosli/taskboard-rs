# Taskboard-API

Back-end web server for [taskboard-rs](https://github.com/christianfosli/taskboard-rs)
using [warp](https://github.com/seanmonstar/warp).

Available as a [Docker
container](https://hub.docker.com/repository/docker/christianfosli/taskboard-api).
Check the docker-compose file in the repository root for how to run with
required dependencies (elasticsearch, ..).

I was also considering [rocket](https://github.com/SergioBenitez/Rocket),
but I decided on warp since rocket did not support asynchronous methods
at the time.
