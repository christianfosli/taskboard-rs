variable "RESOURCE_GROUP" {
  default = "rg-taskboard"
}

variable "K8S_VERSION" {
  default = "1.21.2"
}

variable "AKS_NODE_POOL" {
  default = {
    min_count       = 1
    max_count       = 2
    vm_size         = "Standard_B2s"
    os_disk_size_gb = 32
    max_pods        = 50
  }
}

data "azurerm_resource_group" "rg" {
  name = var.RESOURCE_GROUP
}

# --- The load balancer is provisioned by kubernetes ---

data "azurerm_lb" "aksloadbalancer" {
  name                = "kubernetes"
  resource_group_name = azurerm_kubernetes_cluster.k8s.node_resource_group
}
