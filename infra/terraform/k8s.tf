resource "azurerm_kubernetes_cluster" "k8s" {
  name                = "aks-taskboard"
  location            = azurerm_resource_group.rg.location
  resource_group_name = azurerm_resource_group.rg.name
  dns_prefix          = "aks-taskboard-dns"
  kubernetes_version  = var.KUBERNETES_VERSION

  default_node_pool {
    name                 = "default"
    node_count           = var.AKS_NODE_POOL["node_count"]
    vm_size              = var.AKS_NODE_POOL["vm_size"]
    orchestrator_version = var.KUBERNETES_VERSION
  }

  identity {
    type = "SystemAssigned"
  }
}
