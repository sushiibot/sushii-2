job "traefik" {
  region      = "global"
  datacenters = ["nbg1"]
  type        = "service"

  group "traefik" {
    count = 1

    network {
      port "http" {
        static = 80
      }

      port "https" {
        static = 443
      }

      port "api" {
        static = 8081
      }
    }

    task "traefik" {
      driver = "docker"

      config {
        image        = "traefik:v2.4"
        network_mode = "host"

        volumes = [
          "local/traefik.toml:/etc/traefik/traefik.toml",
        ]
      }

      service {
        name = "traefik"

        check {
          name     = "alive"
          type     = "tcp"
          port     = "http"
          interval = "10s"
          timeout  = "2s"
        }

        tags = [
          "traefik.enable=true",

          # Nomad UI
          "traefik.http.routers.nomad-ui.rule=Host(`nomad.sushii.xyz`)",
          "traefik.http.routers.nomad-ui.service=nomad@consulcatalog",
          "traefik.http.routers.nomad-ui.entrypoints=secure",
          "traefik.http.routers.nomad-ui.tls.certresolver=acme",

          # Consul UI
          "traefik.http.routers.consul-ui.rule=Host(`consul.sushii.xyz`)",
          "traefik.http.routers.consul-ui.service=consul@consulcatalog",
          "traefik.http.routers.consul-ui.entrypoints=secure",
          "traefik.http.routers.consul-ui.tls.certresolver=acme",
        ]
      }

      template {
        data = <<EOF
[entryPoints]
  [entryPoints.insecure]
    address = ":80"
    [entryPoints.insecure.http.redirections]
      [entryPoints.insecure.http.redirections.entryPoint]
        to = "secure"

  [entryPoints.secure]
    address = ":443"
  [entryPoints.traefik]
    address = ":8081"

[api]
  dashboard = true
  insecure  = true

[certificatesResolvers.acme.acme]
  email = "acme@dlee.dev"
  caServer = "https://acme-v02.api.letsencrypt.org/directory"

  [certificatesResolvers.acme.acme.httpChallenge]
    entryPoint = "insecure"

# Enable Consul Catalog configuration backend.
[providers.consulCatalog]
  prefix           = "traefik"
  exposedByDefault = false
  # Only include the http nomad service, otherwise it load balances between
  # the other rpc/serf services causing a gateway error
  constraints      = "!TagRegex(`(rpc|serf)`)"

  [providers.consulCatalog.endpoint]
    address = "127.0.0.1:8500"
    scheme  = "http"

[metrics]
  [metrics.prometheus]
    entryPoint = "traefik"
EOF

        destination = "local/traefik.toml"
      }

      resources {
        cpu    = 100
        memory = 128
      }
    }
  }
}
