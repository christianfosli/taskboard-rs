resource "azurerm_dns_zone" "pubZone" {
  name                = "taskboard.cloud"
  resource_group_name = data.azurerm_resource_group.rg.name
}

//resource "azurerm_dns_a_record" "www" {
//  name                = "www"
//  zone_name           = azurerm_dns_zone.pubZone.name
//  resource_group_name = data.azurerm_resource_group.rg.name
//  ttl                 = 600
//  target_resource_id  = data.azurerm_lb.aksloadbalancer.frontend_ip_configuration.0.public_ip_address_id
//}
//
//resource "azurerm_dns_a_record" "api" {
//  name                = "api"
//  zone_name           = azurerm_dns_zone.pubZone.name
//  resource_group_name = data.azurerm_resource_group.rg.name
//  ttl                 = 600
//  target_resource_id  = data.azurerm_lb.aksloadbalancer.frontend_ip_configuration.0.public_ip_address_id
//}
//
//resource "azurerm_dns_a_record" "metrics" {
//  name                = "metrics"
//  zone_name           = azurerm_dns_zone.pubZone.name
//  resource_group_name = data.azurerm_resource_group.rg.name
//  ttl                 = 600
//  target_resource_id  = data.azurerm_lb.aksloadbalancer.frontend_ip_configuration.0.public_ip_address_id
//}
