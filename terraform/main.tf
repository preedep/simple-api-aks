# Generate random resource group name
resource "random_pet" "rg_name" {
  prefix = var.resource_group_name_prefix
}

resource "azurerm_resource_group" "rg" {
  location = var.resource_group_location
  name     = random_pet.rg_name.id
}

resource "random_pet" "azurerm_kubernetes_cluster_name" {
  prefix = "cluster"
}

resource "random_pet" "azurerm_kubernetes_cluster_dns_prefix" {
  prefix = "dns"
}

resource "azurerm_public_ip" "my_public_ip" {
  name                = "myPublicIP"
  resource_group_name = azurerm_resource_group.rg.name
  location            = "Southeast Asia"
  allocation_method   = "Static"
}

resource "azurerm_kubernetes_cluster" "k8s" {
  location            = azurerm_resource_group.rg.location
  name                = random_pet.azurerm_kubernetes_cluster_name.id
  resource_group_name = azurerm_resource_group.rg.name
  dns_prefix          = random_pet.azurerm_kubernetes_cluster_dns_prefix.id

  identity {
    type = "SystemAssigned"
  }

  default_node_pool {
    name       = "agentpool"
    vm_size    = "Standard_A2_v2"
    node_count = var.node_count
  }
  linux_profile {
    admin_username = var.username

    ssh_key {
      key_data = jsondecode(azapi_resource_action.ssh_public_key_gen.output).publicKey
    }
  }
  network_profile {
    network_plugin = "azure"

    service_cidr = "172.100.0.0/24"

    dns_service_ip = "172.100.0.10"

    docker_bridge_cidr = "172.101.0.1/16"

    load_balancer_sku = "standard"
  }
}
## ---------------------------------------------------
# Managed Prometheus
## ---------------------------------------------------
#resource "azurerm_monitor_workspace" "my_prom" {
#  name                = "prom-aks-workspace"
#  resource_group_name = azurerm_resource_group.rg.name
#  location            = azurerm_resource_group.rg.location
#}
## ---------------------------------------------------
# Managed Grafana
## ---------------------------------------------------
#resource "azurerm_dashboard_grafana" "my_graf" {
#  name                              = "graf-aks-workspace"
#  resource_group_name = azurerm_resource_group.rg.name
#  location            = azurerm_resource_group.rg.location
#  api_key_enabled                   = true
#  deterministic_outbound_ip_enabled = false
#  public_network_access_enabled     = true
#  identity {
#    type = "SystemAssigned"
#  }
#  azure_monitor_workspace_integrations {
#    resource_id = azurerm_monitor_workspace.my_prom.id
#  }
#}

# Add required role assignment over resource group containing the Azure Monitor Workspace
#resource "azurerm_role_assignment" "grafana" {
#  scope                = azurerm_resource_group.rg.id
#  role_definition_name = "Monitoring Reader"
#  principal_id         = azurerm_dashboard_grafana.my_graf.identity[0].principal_id
#}

# Add role assignment to Grafana so an admin user can log in
#resource "azurerm_role_assignment" "grafana-admin" {
#  scope                = azurerm_dashboard_grafana.my_graf.id
#  role_definition_name = "Grafana Admin"
#  principal_id         = var.adminGroupObjectIds[0]
# }

