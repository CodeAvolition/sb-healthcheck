# schulbewerbung.de Health Dashboard Documentation

## Table of Contents

1. [Overview](#overview)
2. [Hardware Setup](#hardware-setup)
3. [Initial Configuration](#initial-configuration)
4. [Architecture & Development](#architecture--development)
5. [Deployment Process](#deployment-process)
6. [Automation & Maintenance](#automation--maintenance)
7. [Network Configuration](#network-configuration)
8. [Troubleshooting](#troubleshooting)
9. [Security](#security)

---

## Overview

### What
A physical health check dashboard for schulbewerbung.de deployments, running on a Raspberry Pi Zero W with a Framework 13 screen.

### Why
- Visual monitoring of production services
- Physical presence in office for team awareness
- Learning exercise in embedded deployment and automation
- Over-engineered for fun and educational value

### Key Features
- Auto-updates every 30 minutes
- Self-healing update mechanism
- Version-synced releases
- Accessible via mDNS (no IP needed)
- Port 80 redirect for clean URLs

---

## Hardware Setup

### Components
- **Raspberry Pi Zero W v1.1** (ARMv6l)
- **Framework 13 original screen** (TBD integration)
- **SD Card** (identified as `sda` in examples)

### Why This Hardware
- **Pi Zero W**: Low power, WiFi built-in, sufficient for dashboard
- **Framework screen**: Reusing existing hardware, good resolution for office display

---

## Initial Configuration

### SD Card Preparation

**What:** Flash Raspberry Pi OS Lite to SD card

**Why:** Headless setup with minimal overhead for dashboard-only device

**Steps:**

1. **Identify SD card**
   ```bash
   lsblk
   ```

2. **Unmount if auto-mounted**
   ```bash
   sudo umount /run/media/codea/boot
   ```

3. **Verify unmounted**
   ```bash
   lsblk | grep sda
   ```

4. **Install and launch imager**
   ```bash
   sudo dnf install rpi-imager
   rpi-imager
   ```

5. **Configure in imager**

   | Setting | Value |
   |---------|-------|
   | Device | Raspberry Pi Zero W v1.1 |
   | OS | Raspberry Pi OS Lite (32-bit) |
   | Storage | sda (your SD card) |

6. **Custom settings (enable SSH from start)**

   | Key | Value |
   |-----|-------|
   | Hostname | sb-healthcheck |
   | Username | sb-dev |
   | Password | <sb-default-dev-pw> |
   | WiFi SSID | <wifi-ssid> |
   | WiFi Password | <wifi-password> |
   | WiFi Country | NL TODO change to DE in GER |
   | Time zone | Europe/Berlin |
   | Keyboard Layout | de |
   | SSH | Enabled |
   | Authentication | Password |

   **Why password first:** Start with working baseline, add SSH keys after verification

### First Connection

**Network Discovery:**
- Method: mDNS via Avahi
- Hostname: `sb-healthcheck.local`
- Router IP: `192.168.1.55`

**SSH Connection:**
```bash
ssh sb-dev@sb-healthcheck.local
```
---

## Architecture & Development

### What
Rust application with background health checker and web dashboard.

### Why Rust
- Memory safety for long-running process
- Excellent async support (tokio)
- Single binary deployment
- Cross-compilation to ARM
- i was looking for a Reason to use it

### Components

**Backend:**
- `src/main.rs` - Entry point, spawns poller and web server
- `src/checker.rs` - Health check polling logic
- `src/cache.rs` - Thread-safe result caching (DashMap)
- `src/models/` - Data structures (config, health checks, status)

**Frontend:**
- `src/server.rs` - Axum web server and HTML dashboard
- Auto-refresh every 5 seconds
- Grid layout (environments → backends/frontends)
- Compact design for 13" screen

**Configuration:**
- `config.json` - Deployment endpoints and check intervals
- Not secret, included in repository

### Local Development

```bash
cargo run
```

Access at `http://localhost:3000`

**Why local testing:** Verify changes before ARM compilation (Pi Zero too slow for native builds)

---

## Deployment Process

### What
GitHub Actions cross-compiles for ARM, Pi pulls pre-built binaries.

### Why This Approach
- Pi Zero W too slow for Rust compilation
- Automated builds ensure consistency
- No manual file copying
- Version control tied to releases

### Creating a Release

**Single command deployment:**
```bash
git tag v0.1.7
git push origin v0.1.7
```

**What happens automatically:**
1. GitHub Actions detects tag push
2. Extracts version from tag (strips 'v')
3. Updates `Cargo.toml` with version during build
4. Cross-compiles for ARM (`arm-unknown-linux-gnueabihf`)
5. Creates GitHub release with binary attached
6. Pi detects new version within 30 minutes

**Why auto-version sync:**
- Single source of truth (git tags)
- No manual `Cargo.toml` editing
- Prevents version mismatches
- Simplifies release to one command

### Initial Pi Setup

**One-time installation:**

```bash
# Download setup files
curl -O https://raw.githubusercontent.com/CodeAvolition/sb-healthcheck/main/update.sh
curl -O https://raw.githubusercontent.com/CodeAvolition/sb-healthcheck/main/sb-healthcheck.service

# Make update script executable
chmod +x update.sh

# Run first update (downloads binary)
./update.sh

# Copy config to installation directory
sudo cp config.json /opt/sb-healthcheck/

# Install systemd service
sudo cp sb-healthcheck.service /etc/systemd/system/
sudo nano /etc/systemd/system/sb-healthcheck.service  # Change User=pi to sb-dev
sudo systemctl daemon-reload
sudo systemctl enable sb-healthcheck
sudo systemctl start sb-healthcheck
```

**Why systemd:**
- Auto-start on boot
- Automatic restart on crashes
- Centralized logging
- Standard Linux service management

---

## Automation & Maintenance

### Auto-Update System

**What:** Pi checks for updates every 30 minutes, installs automatically.

**Why:** Zero-touch maintenance, always running latest code.

#### Self-Updating Script

**Location:** `/home/sb-dev/update.sh`

**What it does:**
1. Downloads itself from GitHub
2. Compares to local version
3. Replaces itself if different
4. Re-executes with new version
5. Checks for binary updates
6. Downloads and installs if newer
7. Restarts service

**Why self-update:** Allows improving the update process without manual intervention.

#### Cron Job

**Current schedule:**
```
*/30 * * * * cd /home/sb-dev && ./update.sh >> /var/log/sb-healthcheck-updates.log 2>&1
```

**Why 30 minutes:** Balance between staying current and not hammering GitHub API.

**View schedule:**
```bash
crontab -l
```

**Edit schedule:**
```bash
crontab -e
```

**Common alternatives:**

| Frequency | Cron Expression |
|-----------|----------------|
| Every 15 min | `*/15 * * * *` |
| Every hour | `0 * * * *` |
| Daily 3 AM | `0 3 * * *` |

**Remove auto-update:**
```bash
crontab -r
```

#### Manual Update

**Force immediate check:**
```bash
cd /home/sb-dev
./update.sh
```

**Why manual updates:**
- Test new releases immediately
- Troubleshoot update issues
- Verify cron behavior

### Version Management

**Check current version:**
```bash
/opt/sb-healthcheck/sb-healthcheck --version
```

**View version history:**
```bash
tail -20 /var/log/sb-healthcheck-updates.log
```

**GitHub releases:**
- Latest: https://github.com/CodeAvolition/sb-healthcheck/releases/latest
- All tags: https://github.com/CodeAvolition/sb-healthcheck/tags

**Why version tracking:**
- Confirms successful updates
- Troubleshooting aid
- Prevents unnecessary restarts

### Log Management

#### Update Logs

**Location:** `/var/log/sb-healthcheck-updates.log`

**What's logged:**
- Update check timestamps
- Current vs available versions
- Download progress
- Installation success/failure
- Service restart confirmation
- Script self-update events

**View logs:**
```bash
tail -f /var/log/sb-healthcheck-updates.log  # Follow live
tail -50 /var/log/sb-healthcheck-updates.log  # Last 50 lines
```

**Rotation:**
- Keeps 2 days of history
- **Why 2 days:** Sufficient for troubleshooting without filling disk
- Automatic cleanup before each update

**Manual cleanup:**
```bash
sudo truncate -s 0 /var/log/sb-healthcheck-updates.log
```

#### Application Logs

**Location:** `/var/log/sb-healthcheck.log`

**What's logged:**
- Application startup with version
- Health check results
- HTTP server events
- Errors and warnings

**View logs:**
```bash
tail -f /var/log/sb-healthcheck.log  # Follow live
journalctl -u sb-healthcheck -f     # Via systemd
```

### Service Management

```bash
# Start/stop/restart
sudo systemctl start sb-healthcheck
sudo systemctl stop sb-healthcheck
sudo systemctl restart sb-healthcheck

# View status
sudo systemctl status sb-healthcheck

# View logs
journalctl -u sb-healthcheck -n 50
```

---

## Network Configuration

### Access URLs

**Primary (mDNS):**
```
http://sb-healthcheck.local/
```

**IP-based:**
```
http://192.168.1.55/
```

**Why mDNS:** No need to remember IP, works across network changes.

### Port 80 Redirect

**What:** Redirects port 80 → 3000 for clean URLs.

**Why:** Standard HTTP port, no `:3000` needed.

**Setup (already configured):**
```bash
# Add redirect rule
sudo nft add table ip nat
sudo nft add chain ip nat prerouting { type nat hook prerouting priority 0 \; }
sudo nft add rule ip nat prerouting tcp dport 80 redirect to :3000

# Make persistent
sudo nft list ruleset | sudo tee /etc/nftables.conf
sudo systemctl enable nftables
sudo systemctl start nftables
```

**Why nftables:** Modern replacement for iptables on Raspberry Pi OS.

### WiFi Reconfiguration

**When:** Moving Pi to different network (e.g., home → office).

**Steps:**

1. Insert SD card into laptop
2. Wait for auto-mount
3. Navigate to boot partition
   ```bash
   cd /run/media/codea/bootfs
   ```
4. Check for config file
   ```bash
   ls -la firstrun.sh
   ```
5. Edit WiFi settings
   ```bash
   sudo nano firstrun.sh
   ```
6. Find WiFi section, update SSID and password
7. Save and exit (Ctrl+X, Y, Enter)
8. Unmount safely
   ```bash
   sudo umount /run/media/codea/bootfs
   ```
9. Insert SD card into Pi and boot

**Why firstrun.sh:** Modern Raspberry Pi OS uses this for initial config. If missing, look for `wpa_supplicant.conf` instead.

---

## Troubleshooting

### Update Not Happening

**Check cron job:**
```bash
crontab -l
```

**Check cron service:**
```bash
sudo systemctl status cron
```

**Check logs:**
```bash
tail -50 /var/log/sb-healthcheck-updates.log
```

**Test manually:**
```bash
cd /home/sb-dev
./update.sh
```

### Wrong Version Showing

**Verify binary version:**
```bash
/opt/sb-healthcheck/sb-healthcheck --version
```

**Check service restart:**
```bash
sudo systemctl status sb-healthcheck
```

**Force restart:**
```bash
sudo systemctl restart sb-healthcheck
```

### Script Not Self-Updating

**Check permissions:**
```bash
ls -la /home/sb-dev/update.sh
# Should show: -rwxr-xr-x
```

**Manual script update:**
```bash
cd /home/sb-dev
curl -O https://raw.githubusercontent.com/CodeAvolition/sb-healthcheck/main/update.sh
chmod +x update.sh
```

### GitHub API Rate Limiting

**Symptom:** Updates fail with API errors.

**Check rate limit:**
```bash
curl -s https://api.github.com/rate_limit
```

**Why it happens:** GitHub limits unauthenticated API calls to 60/hour per IP.

**Solution:** Public releases API has higher limits. If issues persist, add GitHub token to script.

---

## Security

### Permissions Model

**Why sudo is needed:**
- Binary installation to `/opt/` requires root
- Service restart requires root
- Log file writing to `/var/log/` requires root

**File ownership:**
- Update script: `sb-dev`, executable
- Binary: `root`, executable by all
- Log files: Writable by `sb-dev`

### Network Security

- Downloads only from official GitHub releases
- HTTPS enforced for all downloads
- No external dependencies beyond GitHub
- Public repository (no secrets in code)

### Authentication

**Current:** Password-based SSH

**Future:** Add SSH key authentication after initial verification

**Why password first:** Ensures working baseline before layering security.

---

## Quick Reference

### Common Commands

```bash
# Check dashboard status
sudo systemctl status sb-healthcheck

# View live logs
tail -f /var/log/sb-healthcheck.log

# Force update
cd /home/sb-dev && ./update.sh

# Check version
/opt/sb-healthcheck/sb-healthcheck --version

# Restart service
sudo systemctl restart sb-healthcheck

# View cron jobs
crontab -l
```

### Important Paths

| Path | Purpose |
|------|---------|
| `/opt/sb-healthcheck/` | Binary installation directory |
| `/home/sb-dev/update.sh` | Self-updating update script |
| `/var/log/sb-healthcheck.log` | Application logs |
| `/var/log/sb-healthcheck-updates.log` | Update logs |
| `/etc/systemd/system/sb-healthcheck.service` | Service definition |

### Network Details

| Item | Value |
|------|-------|
| Hostname | sb-healthcheck |
| mDNS | sb-healthcheck.local |
| IP Address | 192.168.1.55 |
| HTTP Port | 80 (redirects to 3000) |
| SSH User | sb-dev |

