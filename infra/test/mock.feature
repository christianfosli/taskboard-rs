Feature: Test Feature
  In order to test tf infra
  As developers
  We require a passing test

  Scenario Outline: Azurerm backend
    Given I have azurerm defined
    Then it must contain version
    And its value must match the "2\.5" regex
