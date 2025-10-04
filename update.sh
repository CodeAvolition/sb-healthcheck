#!/bin/bash
set -e

REPO="CodeAvolition/sb-healthcheck"
INSTALL_DIR="/opt/sb-healthcheck"
BINARY="sb-healthcheck"

echo "Fetching latest release..."
LATEST_URL=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep "browser_download_url.*sb-healthcheck" | cut -d '"' -f 4)

echo "Downloading $LATEST_URL..."
curl -L -o /tmp/$BINARY $LATEST_URL

echo "Installing to $INSTALL_DIR..."
sudo mkdir -p $INSTALL_DIR
sudo mv /tmp/$BINARY $INSTALL_DIR/$BINARY
sudo chmod +x $INSTALL_DIR/$BINARY

echo "Restarting service..."
sudo systemctl restart sb-healthcheck

echo "Done! Running version:"
$INSTALL_DIR/$BINARY --version || echo "Binary updated successfully"
