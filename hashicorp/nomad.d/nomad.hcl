datacenter = "nbg1"
data_dir = "/opt/nomad/data"
bind_addr = "10.0.1.1"

consul {
  address = "127.0.0.1:8500"
  tags = ["traefik.enable=true"]
}

vault {
  enabled = true
  address = "http://active.vault.service.consul:8200"
}
