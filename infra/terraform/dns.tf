resource "azurerm_dns_zone" "pubZone" {
  name                = "taskboard.cloud"
  resource_group_name = data.azurerm_resource_group.rg.name
}

resource "azurerm_dns_a_record" "record" {
  name                = "@"
  zone_name           = azurerm_dns_zone.pubZone.name
  resource_group_name = data.azurerm_resource_group.rg.name
  ttl                 = 600
  target_resource_id  = var.K8S_INGRESS_LB_PUBLIC_IP_ID
}
