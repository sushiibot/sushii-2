datacenter = "nbg1"
data_dir = "/opt/consul"
encrypt = ""
ca_file = "/etc/consul.d/consul-agent-ca.pem"
cert_file = "/etc/consul.d/nbg1-server-consul-0.pem"
key_file = "/etc/consul.d/nbg1-server-consul-0-key.pem"
verify_incoming = true
verify_outgoing = true
verify_server_hostname = true
retry_join = ["10.0.1.1"]
acl = {
  enabled = true
  default_policy = "allow"
  enable_token_persistence = true
}
