terraform {
  required_providers {
    hcloud = {
      source = "hetznercloud/hcloud"
      version = "1.26.0"
    }
    helm = {
      source = "hashicorp/helm"
      version = "2.3.0"
    }
  }
}

# Set the variable value in *.tfvars file
# or using the -var="hcloud_token=..." CLI option
variable "hcloud_token" {}
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

module "hcloud_kubernetes_cluster" {
  source          = "JWDobken/kubernetes/hcloud"
  cluster_name    = "demo-cluster"
  hcloud_token    = var.hcloud_token
  hcloud_ssh_keys = [hcloud_ssh_key.default.id]
  master_type     = "cx11" # optional
  worker_type     = "cx11" # optional
  worker_count    = 2
}

output "kubeconfig" {
  value = module.hcloud_kubernetes_cluster.kubeconfig
}

provider "helm" {
  kubernetes {
    host     = module.hcloud_kubernetes_cluster.endpoint

    cluster_ca_certificate = base64decode(module.hcloud_kubernetes_cluster.certificate_authority_data)
    client_certificate     = base64decode(module.hcloud_kubernetes_cluster.client_certificate_data)
    client_key             = base64decode(module.hcloud_kubernetes_cluster.client_key_data)
  }
}

resource "helm_release" "kube_prometheus_stack" {
  name       = "kube-prometheus-stack"

  repository = "https://prometheus-community.github.io/helm-charts"
  chart      = "prometheus-community/kube-prometheus-stack"
}
