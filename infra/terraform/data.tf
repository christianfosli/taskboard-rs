variable "RESOURCE_GROUP" {
  default = "rg-taskboard"
}

variable "K8S_VERSION" {
  default = "1.18.6"
}

variable "AKS_NODE_POOL" {
  default = {
    node_count = 1
    vm_size    = "Standard_B2s"
  }
}

variable "K8S_INGRESS_LB_PUBLIC_IP_ID" {
  type        = string
  description = "Resource ID of public IP for LB for Ingress for k8s cluster"
}

data "azurerm_resource_group" "rg" {
  name = var.RESOURCE_GROUP
}
