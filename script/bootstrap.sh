#!/bin/bash
set -e

PROJECT_ROOT=$(pwd)

echo "[ INFO ] Bootstrapping the System Cooler Daemon ..."

# Install build dependencies
sudo apt update
sudo apt install -y lsb-base

echo "[ INFO ] Bootstrap the System Cooler Daemon complete."