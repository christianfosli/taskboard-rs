terraform {
  backend "azurerm" {
    resource_group_name  = "rg-taskboard"
    storage_account_name = "stchrfostaskboard"
    container_name       = "tfstatek8s"
    key                  = "prod.terraform.tfstate"
  }

  required_providers {
    azurerm = {
      version = "~>2.53"
    }

    helm = {
      version = "~>2.0"
    }

    kubectl = {
      source  = "gavinbunney/kubectl"
      version = "~>1.10"
    }

    kubernetes = {
      version = "~>2.0"
    }

    tls = {
      version = "~>3.1"
    }
  }
}

provider "azurerm" {
  features {}
}

provider "helm" {
  kubernetes {
    config_path    = "~/.kube/config"
    config_context = "aks-taskboard-admin"
  }
}

provider "kubectl" {
  # Replace with official kubernetes provider when alpha version
  # with CRD support matures
  config_path    = "~/.kube/config"
  config_context = "aks-taskboard-admin"
}

provider "kubernetes" {
  config_path    = "~/.kube/config"
  config_context = "aks-taskboard-admin"
}
