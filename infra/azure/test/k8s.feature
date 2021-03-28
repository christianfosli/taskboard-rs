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
      | key                 | value         |
      | name                | aks-taskboard |
      | location            | northeurope   |
      | resource_group_name | rg-taskboard  |
      | kubernetes_version  | 1.19.6        |


  Scenario Outline: AKS Default Node Pool
    Given I have azurerm_kubernetes_cluster defined
    When its address is azurerm_kubernetes_cluster.k8s
    Then it must contain default_node_pool
    And it must contain <key>
    And its value must be <value>

    Examples:
      | key                  | value        |
      | node_count           | 1            |
      | vm_size              | Standard_B2s |
      | orchestrator_version | 1.19.6       |
      | os_disk_size_gb      | 32           |
      | max_pods             | 50           |


  Scenario: AKS Network Profile should use cheap load balancer
    Given I have azurerm_kubernetes_cluster defined
    When its address is azurerm_kubernetes_cluster.k8s
    Then it must contain network_profile
    And it must contain load_balancer_sku
    And its value must be Basic
