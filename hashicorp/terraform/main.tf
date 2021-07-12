terraform {
  required_providers {
    hcloud = {
      source = "hetznercloud/hcloud"
      version = "1.26.0"
    }
    nomad = {
      source = "hashicorp/nomad"
      version = "1.4.14"
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

resource "hcloud_server" "hashi_node" {
  name        = "node1-hashi"
  server_type = "cx11"
  image       = "ubuntu-20.04"
  location    = "nbg1"
  ssh_keys    = [hcloud_ssh_key.default.id]

  network {
    network_id = hcloud_network.network.id
    ip         = "10.0.1.1"
  }

  connection {
    type  = "ssh"
    user  = "root"
    agent = true
    host  = self.ipv4_address
  }

  provisioner "file" {
    source      = var.ssh_pub_key
    destination = "/tmp/id_rsa.pub"
  }

  provisioner "file" {
    source      = "setup.sh"
    destination = "/tmp/setup.sh"
  }

  provisioner "remote-exec" {
    inline = [
      "chmod +x /tmp/setup.sh",
      "/tmp/setup.sh",
    ]
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

  connection {
    type  = "ssh"
    user  = "root"
    agent = true
    host  = self.ipv4_address
  }

  provisioner "file" {
    source      = var.ssh_pub_key
    destination = "/tmp/id_rsa.pub"
  }

  provisioner "file" {
    source      = "setup.sh"
    destination = "/tmp/setup.sh"
  }

  provisioner "remote-exec" {
    inline = [
      "chmod +x /tmp/setup.sh",
      "/tmp/setup.sh",
    ]
  }

  depends_on = [
    hcloud_network_subnet.network-subnet
  ]
}
