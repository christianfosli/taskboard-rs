# Taskboard Infra

Divided into two:

* azure: Azure resources which are *not* managed by Kubernetes,
  includes provisioning of Azure Kubernetes Service cluster

* k8s: Resources/infrastructure that live inside the kubernetes cluster

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

* Create a storage account and containers, to keep Terraform's state

  * Check/update terraform/main.tf
