variable "RESOURCE_GROUP" {
  default = "rg-taskboard"
}

variable "STORAGE_ACCOUNT" {
  default = "stchrfostaskboard"
}

variable "KUBERNETES_VERSION" {
  default = "1.18.6"
}

variable "AKS_NODE_POOL" {
  default = {
    node_count = 1
    vm_size    = "Standard_B1s"
  }
}

data "azurerm_resource_group" "rg" {
  name = var.RESOURCE_GROUP
}
