ui = true

storage "raft" {
  path = "/opt/raft"
  node_id = "raft_node_1"
}

# HTTPS listener
listener "tcp" {
  address       = "10.0.1.1:8200"
  tls_cert_file = "/opt/vault/tls/tls.crt"
  tls_key_file  = "/opt/vault/tls/tls.key"
}

service_registration "consul" {
  address      = "10.0.1.1:8500"
}

api_addr = "http://10.0.1.1:8200"
cluster_addr = "http://10.0.1.1:8201"
