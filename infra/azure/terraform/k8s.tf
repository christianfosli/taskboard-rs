resource "azurerm_kubernetes_cluster" "k8s" {
  name                = "aks-taskboard"
  location            = data.azurerm_resource_group.rg.location
  resource_group_name = data.azurerm_resource_group.rg.name
  dns_prefix          = "aks-taskboard-dns"
  kubernetes_version  = var.K8S_VERSION

  default_node_pool {
    name                 = "default"
    node_count           = var.AKS_NODE_POOL["node_count"]
    vm_size              = var.AKS_NODE_POOL["vm_size"]
    os_disk_size_gb      = var.AKS_NODE_POOL["os_disk_size_gb"]
    orchestrator_version = var.K8S_VERSION
  }

  network_profile {
    load_balancer_sku = "Basic"
  }

  identity {
    type = "SystemAssigned"
  }
}
