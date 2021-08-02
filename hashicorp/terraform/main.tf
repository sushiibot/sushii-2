terraform {
  required_providers {
    hcloud = {
      source = "hetznercloud/hcloud"
      version = "1.26.0"
    }
  }
}

# Set the variable value in *.tfvars file
# or using the -var="hcloud_token=..." CLI option
variable "hcloud_token" {}
variable "ssh_priv_key" {
  type    = string
  default = "~/.ssh/id_rsa"
}
variable "ssh_pub_key" {
  type    = string
  default = "~/.ssh/id_rsa.pub"
}
variable "cloud_init_script" {
  type    = string
  default = "cloud-init.yaml"
}

########################################
# Configure the Hetzner Cloud Provider #
provider "hcloud" {
  token = var.hcloud_token
}

# Create a new SSH key
resource "hcloud_ssh_key" "default" {
  name = "sushii"
  public_key = file(var.ssh_pub_key)
}

resource "hcloud_network" "network" {
  name     = "network"
  ip_range = "10.0.0.0/16"
}

resource "hcloud_network_subnet" "network-subnet" {
  type         = "cloud"
  network_id   = hcloud_network.network.id
  network_zone = "eu-central"
  ip_range     = "10.0.1.0/24"
}

# cloud-init script
data "template_file" "user_data" {
  template = file(var.cloud_init_script)
}

resource "hcloud_server" "hashi_node" {
  name        = "node1-hashi"
  server_type = "cx11"
  image       = "ubuntu-20.04"
  location    = "nbg1"
  ssh_keys    = [hcloud_ssh_key.default.id]
  user_data   = data.template_file.user_data

  network {
    network_id = hcloud_network.network.id
    ip         = "10.0.1.1"
  }

  # **Note**: the depends_on is important when directly attaching the
  # server to a network. Otherwise Terraform will attempt to create
  # server and sub-network in parallel. This may result in the server
  # creation failing randomly.
  depends_on = [
    hcloud_network_subnet.network-subnet
  ]
}

resource "hcloud_server" "sushii_web" {
  name        = "node2-web"
  server_type = "cx11"
  image       = "ubuntu-20.04"
  location    = "nbg1"
  ssh_keys    = [hcloud_ssh_key.default.id]

  network {
    network_id = hcloud_network.network.id
    ip         = "10.0.1.2"
  }

  depends_on = [
    hcloud_network_subnet.network-subnet
  ]
}
