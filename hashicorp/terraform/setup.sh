#!/bin/bash
set -x

### Dependencies and stuff
sudo DEBIAN_FRONTEND=noninteractive apt-get -y -o Dpkg::Options::="--force-confdef" -o Dpkg::Options::="--force-confold" dist-upgrade

# HashiCorp repository
curl -fsSL https://apt.releases.hashicorp.com/gpg | sudo apt-key add -
sudo apt-add-repository "deb [arch=amd64] https://apt.releases.hashicorp.com $(lsb_release -cs) main"

# Docker repository
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo \
    "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
    $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# Install deps, hashi stuff, docker
sudo apt-get update -y
sudo apt-get -y install curl wget git vim apt-transport-https ca-certificates gnupg lsb-release \
    consul nomad \
    docker-ce docker-ce-cli containerd.io

# Setup sudo to allow no-password sudo for "hashicorp" group and adding "terraform" user
sudo groupadd -r hashicorp
sudo useradd -m -s /bin/bash drk
sudo usermod -a -G hashicorp drk
sudo cp /etc/sudoers /etc/sudoers.orig
echo "drk  ALL=(ALL) NOPASSWD:ALL" | sudo tee /etc/sudoers.d/drk

# Installing SSH key
sudo mkdir -p /home/drk/.ssh
sudo chmod 700 /home/drk/.ssh
sudo cp /root/.ssh/authorized_keys /home/drk/.ssh/authorized_keys
sudo chmod 600 /home/drk/.ssh/authorized_keys
sudo chown -R drk /home/drk/.ssh
sudo usermod --shell /bin/bash drk

