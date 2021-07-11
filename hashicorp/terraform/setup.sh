#!/bin/bash
set -x

# Install necessary dependencies
sudo DEBIAN_FRONTEND=noninteractive apt-get -y -o Dpkg::Options::="--force-confdef" -o Dpkg::Options::="--force-confold" dist-upgrade
sudo apt-get update -y
sudo apt-get -y install curl wget git vim apt-transport-https ca-certificates

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

