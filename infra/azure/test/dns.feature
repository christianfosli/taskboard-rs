Feature: DNS and Custom Domain
  In order to make the cluster available to the public
  As developers
  We require DNS for a custom domain


  Scenario Outline: DNS Zone
    Given I have azurerm_dns_zone defined
    When its address is azurerm_dns_zone.pubZone
    Then it must contain <key>
    And its value must be <value>

    Examples:
      | key                 | value           |
      | name                | taskboard.cloud |
      | resource_group_name | rg-taskboard    |


