resource "azurerm_dns_zone" "pubZone" {
  name                = "taskboard.cloud"
  resource_group_name = data.azurerm_resource_group.rg.name
}

resource "azurerm_dns_a_record" "record" {
  name                = "@"
  zone_name           = azurerm_dns_zone.pubZone.name
  resource_group_name = data.azurerm_resource_group.rg.name
  ttl                 = 600
  target_resource_id  = data.azurerm_lb.aksloadbalancer.frontend_ip_configuration.1.public_ip_address_id
}
