packer {
  required_plugins {
    hcloud = {
      version = ">= 1.0.0"
      source  = "github.com/hashicorp/hcloud"
    }
  }
}

variable "hcloud_token" {
  type    = string
  default = "${env("HCLOUD_TOKEN")}"
}

locals {
  timestamp = regex_replace(timestamp(), "[- TZ:]", "")
  time      = timestamp()
}

# source "docker" "test" {
#     image = "ubuntu"
#     discard = true
# }
# 
# build {
#   sources = [
#     "source.docker.test"
#   ]
# 
#   provisioner "shell" {
#     inline = ["apt-get update && apt-get install -y python3 sudo ca-certificates"]
#   }
# 
#   provisioner "ansible" {
#     playbook_file = "./ansible/main.yml"
#   }
# }


source "hcloud" "hashi" {
  token       = "${var.hcloud_token}"
  image       = "ubuntu-20.04"
  location    = "ash"
  server_type = "cpx11"

  server_name = "packer-hashi"
  snapshot_name = "hashi-${local.timestamp}"
  snapshot_labels = {
    name = "hashi"
  }

  ssh_username = "root"
}

build {
  sources = [
    "source.hcloud.hashi"
  ]

  provisioner "ansible" {
    playbook_file = "./ansible/main.yml"
  }
}
