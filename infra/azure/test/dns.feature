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


  Scenario Outline: DNS A Record - root alias to www
    Given I have azurerm_dns_a_record defined
    When its address is azurerm_dns_a_record.root
    Then it must contain <key>
    And its value must be <value>

    Examples:
      | key                 | value           |
      | name                | @               |
      | zone_name           | taskboard.cloud |
      | resource_group_name | rg-taskboard    |
      | ttl                 | 600             |


  Scenario Outline: DNS A Record - www
    Given I have azurerm_dns_a_record defined
    When its address is azurerm_dns_a_record.www
    Then it must contain <key>
    And its value must be <value>

    Examples:
      | key                 | value           |
      | name                | www             |
      | zone_name           | taskboard.cloud |
      | resource_group_name | rg-taskboard    |
      | ttl                 | 600             |

  Scenario: DNS A Record - www - must include resource id of cluster ingress LB
    Given I have azurerm_dns_a_record defined
    When its address is azurerm_dns_a_record.www
    And it has target_resource_id
    Then it must contain target_resource_id
    And its value must match the "^.+kubernetes.+$" regex


  Scenario Outline: DNS A Record - api
    Given I have azurerm_dns_a_record defined
    When its address is azurerm_dns_a_record.api
    Then it must contain <key>
    And its value must be <value>

    Examples:
      | key                 | value           |
      | name                | api             |
      | zone_name           | taskboard.cloud |
      | resource_group_name | rg-taskboard    |
      | ttl                 | 600             |

  Scenario: DNS A Record - api - must include resource id of cluster ingress LB
    Given I have azurerm_dns_a_record defined
    When its address is azurerm_dns_a_record.api
    And it has target_resource_id
    Then it must contain target_resource_id
    And its value must match the "^.+kubernetes.+$" regex


  Scenario Outline: DNS A Record - metrics
    Given I have azurerm_dns_a_record defined
    When its address is azurerm_dns_a_record.metrics
    Then it must contain <key>
    And its value must be <value>

    Examples:
      | key                 | value           |
      | name                | metrics         |
      | zone_name           | taskboard.cloud |
      | resource_group_name | rg-taskboard    |
      | ttl                 | 600             |

  Scenario: DNS A Record - metrics - must include resource id of cluster ingress LB
    Given I have azurerm_dns_a_record defined
    When its address is azurerm_dns_a_record.metrics
    And it has target_resource_id
    Then it must contain target_resource_id
    And its value must match the "^.+kubernetes.+$" regex
