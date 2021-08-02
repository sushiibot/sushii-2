job "bp-modmail" {
  region      = "global"
  datacenters = ["nbg1"]
  type        = "service"

  group "modmail" {
    count = 1

    task "modmail" {
      driver = "docker"

      config {
        image = "https://github.com/didinele/modmailbot.git"

        volumes = [
          "local/config.ini:/SOMETHING/config.ini",
        ]
      }

      service {
        name = "modmail"
      }

      template {
        data = <<EOF
# Required settings
# -----------------
{{ with secret "kv/discord/modmail/blackpink" }}
token = {{.Data.DISCORD_TOKEN}}
{{ end }}
mainServerId = 187450744427773963
inboxServerId = 187450744427773963
logChannelId = 783444823234838558
attachmentStorageChannelId = 783444823234838558

categoryAutomation.newThread = 783444822354296873

# Common settings
# ----------------------------------
prefix = !
inboxServerPermission = banMembers
status = Message me for help!
responseMessage = Thank you for your message! We will reply to you here as soon as possible. If you are reporting a user, please provide screenshots and their ID if you haven't already!

EOF

        destination = "local/config.ini"
      }

      resources {
        cpu    = 100
        memory = 128
      }
    }
  }
}
