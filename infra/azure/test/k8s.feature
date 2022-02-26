Feature: Azure Kubernetes Service
  In order to host applications/services
  As developers
  We require an Azure Kubernetes Service


  Scenario Outline: AKS Cluster
    Given I have azurerm_kubernetes_cluster defined
    When its address is azurerm_kubernetes_cluster.k8s
    Then it must contain <key>
    And its value must be <value>

    Examples:
      | key                       | value         |
      | name                      | aks-taskboard |
      | location                  | northeurope   |
      | resource_group_name       | rg-taskboard  |
      | automatic_channel_upgrade | patch         |


  Scenario Outline: AKS Default Node Pool
    Given I have azurerm_kubernetes_cluster defined
    When its address is azurerm_kubernetes_cluster.k8s
    Then it must contain default_node_pool
    And it must contain <key>
    And its value must be <value>

    Examples:
      | key                  | value        |
      | min_count            | 1            |
      | max_count            | 2            |
      | vm_size              | Standard_B2s |
      | os_disk_size_gb      | 32           |
      | max_pods             | 50           |


  Scenario: AKS Network Profile should use cheap load balancer
    Given I have azurerm_kubernetes_cluster defined
    When its address is azurerm_kubernetes_cluster.k8s
    Then it must contain network_profile
    And it must contain load_balancer_sku
    And its value must be Basic


  Scenario: AKS role based access control enabled
    Given I have azurerm_kubernetes_cluster defined
    When its address is azurerm_kubernetes_cluster.k8s
    Then it must contain role_based_access_control
    And it must contain enabled
    And its value must be true


  Scenario: AKS role based access control - Azure AD rbac enabled
    Given I have azurerm_kubernetes_cluster defined
    When its address is azurerm_kubernetes_cluster.k8s
    Then it must contain role_based_access_control
    And it must contain azure_active_directory
    And it must contain azure_rbac_enabled
    And its value must be true
