terraform {
  backend "azurerm" {
    resource_group_name  = var.RESOURCE_GROUP
    storage_account_name = var.STORAGE_ACCOUNT
    container_name       = "tfstate"
    key                  = "prod.terraform.tfstate"
  }
}

provider "azurerm" {
  version = "~>2.5"
  features {}
}
