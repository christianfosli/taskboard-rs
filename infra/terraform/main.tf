terraform {
  backend "azurerm" {
    resource_group_name  = "rg-taskboard"
    storage_account_name = "stchrfostaskboard"
    container_name       = "tfstate"
    key                  = "prod.terraform.tfstate"
  }
}

provider "azurerm" {
  version = "~>2.5"
  features {}
}
