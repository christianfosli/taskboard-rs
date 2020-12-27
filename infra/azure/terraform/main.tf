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
  }
}

provider "azurerm" {
  features {}
}
