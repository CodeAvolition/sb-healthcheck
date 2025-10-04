#!/bin/bash
set -e

REPO="CodeAvolition/sb-healthcheck"
INSTALL_DIR="/opt/sb-healthcheck"
BINARY="sb-healthcheck"

echo "Fetching latest release..."
LATEST_URL=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep "browser_download_url.*sb-healthcheck" | cut -d '"' -f 4)

if [ -z "$LATEST_URL" ]; then
    echo "Error: Could not find release URL"
    exit 1
fi

echo "Downloading $LATEST_URL..."
curl -L -o /tmp/$BINARY "$LATEST_URL"

echo "Installing to $INSTALL_DIR..."
sudo mkdir -p $INSTALL_DIR
sudo mv /tmp/$BINARY $INSTALL_DIR/$BINARY
sudo chmod +x $INSTALL_DIR/$BINARY

if systemctl is-active --quiet sb-healthcheck; then
    echo "Restarting service..."
    sudo systemctl restart sb-healthcheck
else
    echo "Service not running, skipping restart"
fi

echo "✓ Update complete!"
