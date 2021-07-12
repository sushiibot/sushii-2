job "hashi-ui" {
  region      = "global"
  datacenters = ["nbg1"]
  type        = "service"

  group "hashi-ui" {
    count = 1

    network {
      port "http" {
        to = 3000
        host_network = "private"
      }
    }

    task "hashi-ui" {
      driver = "docker"

      config {
        image       = "jippi/hashi-ui"
        dns_servers = ["${attr.unique.network.ip-address}"]
        ports = [
          "http"
        ]
      }

      env {
        NOMAD_ENABLE  = "1"
        NOMAD_ADDR    = "http://http.nomad.service.consul:4646"
        CONSUL_ENABLE = "1"
        CONSUL_ADDR   = "consul.service.consul:8500"
        PROXY_ADDRESS = "https://hashi-ui.sushii.xyz"
      }

      service {
        name = "hashi-ui"
        port = "http"

        check {
          type     = "http"
          path     = "/"
          interval = "10s"
          timeout  = "2s"
        }

        tags = [
          "traefik.enable=true",
          "traefik.http.routers.hashi-ui.rule=Host(`hashi-ui.sushii.xyz`)",
          "traefik.http.routers.hashi-ui.entrypoints=secure",
          "traefik.http.routers.hashi-ui.tls.certresolver=acme",
        ]
      }

      resources {
        cpu    = 500
        memory = 512
      }
    }
  }
}
