# Taskboard Infra

## Prerequisites

There are a few steps which are not included in pipelines which must be run
manually if recreating infra from scratch:

* Create a service principal for the pipelines hosted agent.

  * Set/update related environment variables (see pipeline).

* Create a storage account and container, to keep Terraform's state

  * Set/update related environment variables (see pipeline)
