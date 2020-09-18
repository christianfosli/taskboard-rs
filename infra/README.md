# Taskboard Infra

## Prerequisites

### Running locally

* Be signed in with `az cli` to an account with sufficient permissions

* Have Terraform installed, preferably the same version that GitHub's
  ubuntu-latest image has.

### One-Time actions

There are a few steps which are not included in pipelines which must be run
manually if recreating infra from scratch

* Create a service principal for the pipelines hosted agent.

  * Set/update related environment variables (see pipeline).

* Create a storage account and container, to keep Terraform's state

  * Check/update terraform/main.tf

### Other manual steps

The manifests in the k8s folder must be manually applied,
using `kubectl apply`, or the applicable script, for now.
