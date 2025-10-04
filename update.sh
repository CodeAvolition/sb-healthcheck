#!/bin/bash
set -e

REPO="CodeAvolition/sb-healthcheck"
INSTALL_DIR="/opt/sb-healthcheck"
BINARY="sb-healthcheck"
LOG_FILE="/var/log/sb-healthcheck-updates.log"

# Function to log with timestamp
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | sudo tee -a "$LOG_FILE"
}

# Get current version if binary exists
CURRENT_VERSION="none"
if [ -f "$INSTALL_DIR/$BINARY" ]; then
    CURRENT_VERSION=$($INSTALL_DIR/$BINARY --version 2>/dev/null || echo "unknown")
fi

log "=== Update check started ==="
log "Current version: $CURRENT_VERSION"

echo "Fetching latest release..."
LATEST_RELEASE=$(curl -s https://api.github.com/repos/$REPO/releases/latest)
LATEST_TAG=$(echo "$LATEST_RELEASE" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
LATEST_URL=$(echo "$LATEST_RELEASE" | grep "browser_download_url.*sb-healthcheck" | cut -d '"' -f 4)

if [ -z "$LATEST_URL" ]; then
    log "ERROR: Could not find release URL"
    exit 1
fi

log "Latest available version: $LATEST_TAG"

# Check if update needed
if [ "$CURRENT_VERSION" = "$LATEST_TAG" ]; then
    log "Already on latest version, no update needed"
    exit 0
fi

log "Update available: $CURRENT_VERSION -> $LATEST_TAG"
log "Downloading from: $LATEST_URL"

echo "Downloading $LATEST_URL..."
curl -L -o /tmp/$BINARY "$LATEST_URL"

echo "Installing to $INSTALL_DIR..."
sudo mkdir -p $INSTALL_DIR
sudo mv /tmp/$BINARY $INSTALL_DIR/$BINARY
sudo chmod +x $INSTALL_DIR/$BINARY

log "Binary installed successfully"

if systemctl is-active --quiet sb-healthcheck; then
    log "Restarting service..."
    sudo systemctl restart sb-healthcheck
    log "Service restarted"
else
    log "Service not running, skipping restart"
fi

log "✓ Update complete: $LATEST_TAG"
log "=== Update check finished ==="
