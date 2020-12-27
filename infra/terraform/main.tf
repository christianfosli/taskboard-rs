terraform {
  backend "azurerm" {
    resource_group_name  = "rg-taskboard"
    storage_account_name = "stchrfostaskboard"
    container_name       = "tfstate"
    key                  = "prod.terraform.tfstate"
  }

  required_providers {
    azurerm = {
      version = "~>2.41"
    }

    helm = {
      version = "~>2.0"
    }

    kubectl = {
      # Replace with official kubernetes provider when alpha version
      # with CRD support matures
      source  = "gavinbunney/kubectl"
      version = "~>1.9"
    }
  }
}

provider "azurerm" {
  features {}
}
