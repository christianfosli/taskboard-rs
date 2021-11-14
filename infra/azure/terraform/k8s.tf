resource "azurerm_kubernetes_cluster" "k8s" {
  name                      = "aks-taskboard"
  location                  = data.azurerm_resource_group.rg.location
  resource_group_name       = data.azurerm_resource_group.rg.name
  dns_prefix                = "aks-taskboard-dns"
  kubernetes_version        = var.K8S_VERSION
  automatic_channel_upgrade = "patch"

  default_node_pool {
    name                 = "default"
    enable_auto_scaling  = true
    min_count            = var.AKS_NODE_POOL["min_count"]
    max_count            = var.AKS_NODE_POOL["max_count"]
    vm_size              = var.AKS_NODE_POOL["vm_size"]
    os_disk_size_gb      = var.AKS_NODE_POOL["os_disk_size_gb"]
    max_pods             = var.AKS_NODE_POOL["max_pods"]
    orchestrator_version = var.K8S_VERSION
  }

  network_profile {
    network_plugin    = "azure"
    load_balancer_sku = "Basic"
  }

  identity {
    type = "SystemAssigned"
  }
}
