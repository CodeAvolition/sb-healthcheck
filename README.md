# schulbewerbung.de health dashboard
- im building a health dashboard like the one we had previously vibe-coded, but i want to overengineer it and put it on a physical device, which i can set up in the office.

## The Setup:
- raspberry pi zero w v1.1
- framework 13 original screen
- tbd.

## Documentation
### The raspberry pi zero w v1.1 (PiZW)
- Identify SD card
run `lsblk`
- Unmount if auto-mounted
`sudo umount /run/media/codea/boot`
- Verify unmounted
`lsblk | grep sda`
- install and use rpi-imager
`sudo dnf install rpi-imager`
`rpi-imager`
- choose device, os and Storage
Device: raspberry pi zero w v1.1
Os: Raspberry Pi OS Lite (32-bit)
Storage: sda (whatever it was called in the imager)
- when clicking next i get to choose custom optionssystemctl status avahi-daemon
- we want to do those so ssh etc. works from the get go

| Key             | Value          |
| --------------- | -------------- |
| Hostname        | sb-healthcheck |
| Username        | sb-dev         |
| Password        | 123QWEasd!     |
| WiFi-SSID       | FTTH-A0AC      |
| WiFi-Password   | 7EDG65d4       |
| WiFi-Country    | NL             |
| Time zone       | Europe/Berlin  |
| Keyboard-Layout | de             | 

- Service Settings:

| Key            | Value                                                          | 
| -------------- | -------------------------------------------------------------- |
| SSH            | Enabled                                                        |
| Authentication | Password (will add SSH keys after first successful connection) |

Start with working baseline, layer in security after verification


#### Network Discovery
- Method: mDNS via Avahi
- Hostname: sb-healthcheck.local
- SSH command: `ssh sb-dev@sb-healthcheck.local`
- said yes to allow the connection
- reconnected after the fingerprint was saved and the first connection was closed

#### First successful SSH connection
- Date: 2025-10-04
- IP assigned by router: 192.168.1.55
- SSH fingerprint: SHA256:Q01DJFaAYhE12rq2YpBqqmDO9/xQid4M5p1PJUIxswM
- Command: `ssh sb-dev@sb-healthcheck.local`
- Kernel: 6.12.47+rpt-rpi-v6 (ARMv6l)

#### Dashboard in Rust
- Backend task that polls all the healthchecks and caches it.
- axum server that serves a static html page that dispays that cached data

### sb-healthcheck Deployment Process

## Overview
Health check dashboard for schulbewerbung.de deployments. Runs on Raspberry Pi Zero W, built via GitHub Actions.

## Architecture
- **Backend**: Rust (axum web server)
- **Frontend**: Minimal HTML/CSS with auto-refresh
- **Deployment**: GitHub Actions cross-compiles for ARM, Pi pulls binary from releases

## Local Development

### Running locally
```bash
cargo run
````

Access at `http://localhost:3000`

### Project structure

- `src/main.rs` - Entry point, spawns poller and web server
- `src/models/` - Data structures (config, health checks, status)
- `src/checker.rs` - Health check polling logic
- `src/cache.rs` - Thread-safe result caching with DashMap
- `src/server.rs` - Axum web server and HTML dashboard
- `config.json` - Deployment configuration

## Deployment Workflow

### Creating a new release
```Bash
# Make your changes 
git add . 
git commit -m Description of changes; 
git push
git tag v0.X.Y 
git push origin v0.X.Y` # uses sem ver
```
### What happens automatically
1. GitHub Actions detects tag push
2. Cross-compiles for ARM (arm-unknown-linux-gnueabihf)
3. Creates GitHub release with binary attached
4. Pi can pull latest release


- v0.1.3 went through after working on the github release pipeline "for a bit" 


## Pi Setup (Complete)

### Initial Installation
# Download setup files
curl -O https://raw.githubusercontent.com/CodeAvolition/sb-healthcheck/main/update.sh
curl -O https://raw.githubusercontent.com/CodeAvolition/sb-healthcheck/main/sb-healthcheck.service

# Make update script executable
chmod +x update.sh

# Run first update (downloads binary)
./update.sh

# Copy config.json to installation directory
sudo cp config.json /opt/sb-healthcheck/

# Install and start systemd service
sudo cp sb-healthcheck.service /etc/systemd/system/
sudo nano /etc/systemd/system/sb-healthcheck.service  # Change User=pi to your username
sudo systemctl daemon-reload
sudo systemctl enable sb-healthcheck
sudo systemctl start sb-healthcheck

### Manual Updates

Bash

`./update.sh`

### Service Management

Bash

`# Start/stop/restart sudo systemctl start sb-healthcheck sudo systemctl stop sb-healthcheck sudo systemctl restart sb-healthcheck  # View logs sudo journalctl -u sb-healthcheck -n 50`


## Accessible via:
http://sb-healthcheck.local/
much nicer than typing out the ip


# Automation & Maintenance

This document explains the automated systems that keep the health check dashboard updated and running smoothly.

## Auto-Update System

### Overview
The Raspberry Pi automatically checks for and installs new versions every 30 minutes. This ensures the dashboard always runs the latest code without manual intervention.

### How It Works

**1. Self-Updating Script (`update.sh`)**
- Downloads itself from GitHub before checking for binary updates
- **Why:** Allows us to improve the update process itself without manual intervention
- Compares local script to remote version
- If different, replaces itself and re-executes
- **Location:** `/home/sb-dev/update.sh`

**2. Binary Update Process**
- Fetches latest release from GitHub API
- Compares current version (from `--version` flag) with latest tag
- Downloads new binary if versions differ
- Installs to `/opt/sb-healthcheck/`
- Restarts systemd service automatically
- **Why restart:** Ensures new code runs immediately without manual service management

**3. Cron Job Scheduler**
- Runs `update.sh` every 30 minutes
- Logs all activity to `/var/log/sb-healthcheck-updates.log`
- **Why 30 minutes:** Balance between staying current and not hammering GitHub API

### Managing the Cron Job

**View current schedule:**
crontab -l

**Edit schedule:**

Bash

`crontab -e`

Current entry:

Plaintext

`*/30 * * * * cd /home/sb-dev &amp;&amp; ./update.sh &gt;&gt; /var/log/sb-healthcheck-updates.log 2&gt;&amp;1`

**Cron syntax breakdown:**

- `*/30` - Every 30 minutes
- `* * * *` - Every hour, day, month, day-of-week
- `cd /home/sb-dev` - Change to script directory (important for relative paths)
- `./update.sh` - Run the update script
- `>> /var/log/...` - Append output to log file
- `2>&1` - Redirect errors to same log file

**Common schedule changes:**

Every hour:

Plaintext

`0 * * * * cd /home/sb-dev &amp;&amp; ./update.sh &gt;&gt; /var/log/sb-healthcheck-updates.log 2&gt;&amp;1`

Every 15 minutes:

Plaintext

`*/15 * * * * cd /home/sb-dev &amp;&amp; ./update.sh &gt;&gt; /var/log/sb-healthcheck-updates.log 2&gt;&amp;1`

Daily at 3 AM:

Plaintext

`0 3 * * * cd /home/sb-dev &amp;&amp; ./update.sh &gt;&gt; /var/log/sb-healthcheck-updates.log 2&gt;&amp;1`

**Remove auto-update:**

Bash

`crontab -r  # Removes all cron jobs for current user`

### Manual Update

Force an update check immediately:

Bash

`cd /home/sb-dev ./update.sh`

**Why manual updates are useful:**

- Testing new releases immediately
- Troubleshooting update issues
- Verifying cron job behavior

## Version Management

### Auto-Versioning in CI/CD

**How versions are determined:**

1. Developer creates git tag: `git tag v0.1.7`
2. GitHub Actions workflow triggers on tag push
3. Workflow extracts version from tag (strips 'v' prefix)
4. `sed` command updates `Cargo.toml` during build
5. Binary is compiled with correct version embedded
6. Release is created with tag name

**Why this approach:**

- Single source of truth: git tags
- No manual `Cargo.toml` editing needed
- Prevents version mismatch between tag and binary
- Simplifies release process to one command

### Creating a New Release

Bash

`# Create and push tag (triggers build automatically) git tag v0.1.7 git push origin v0.1.7`

**What happens next:**

1. GitHub Actions starts build (~3-5 minutes)
2. ARM binary is cross-compiled
3. Release is created with binary attached
4. Next cron run (within 30 min) detects and installs it

### Version Detection

The `--version` flag reads from `CARGO_PKG_VERSION`, which comes from `Cargo.toml`:

Bash

`/opt/sb-healthcheck/sb-healthcheck --version # Output: v0.1.7`

**Why version detection matters:**

- Update script compares versions to avoid unnecessary restarts
- Logs show version history for troubleshooting
- Confirms successful updates

### Checking Current Version

**On the Pi:**

Bash

`/opt/sb-healthcheck/sb-healthcheck --version`

**In logs:**

Bash

`tail -20 /var/log/sb-healthcheck-updates.log`

**On GitHub:**

- Check latest release: https://github.com/CodeAvolition/sb-healthcheck/releases/latest
- View all tags: https://github.com/CodeAvolition/sb-healthcheck/tags

## Log Management

### Update Logs

**Location:** `/var/log/sb-healthcheck-updates.log`

**What's logged:**

- Update check start/finish timestamps
- Current and available versions
- Download progress
- Installation success/failure
- Service restart confirmation
- Script self-update events

**View recent activity:**

Bash

`tail -f /var/log/sb-healthcheck-updates.log  # Follow live tail -50 /var/log/sb-healthcheck-updates.log  # Last 50 lines`

**Log rotation:**

- Keeps only 2 days of history
- **Why 2 days:** Enough for troubleshooting recent issues without filling disk
- Configured in `update.sh` via `find` command
- Runs before each update check

**Manual log cleanup:**

Bash

`# Clear all update logs sudo truncate -s 0 /var/log/sb-healthcheck-updates.log  # Or remove entirely (will be recreated) sudo rm /var/log/sb-healthcheck-updates.log`

### Application Logs

**Location:** `/var/log/sb-healthcheck.log`

**What's logged:**

- Application startup with version
- Health check results
- HTTP server events
- Errors and warnings

**View logs:**

Bash

`tail -f /var/log/sb-healthcheck.log  # Follow live journalctl -u sb-healthcheck -f     # Via systemd`

## Troubleshooting

### Update Not Happening

**Check cron job exists:**

Bash

`crontab -l`

**Check cron service running:**

Bash

`sudo systemctl status cron`

**Check update logs:**

Bash

`tail -50 /var/log/sb-healthcheck-updates.log`

**Test update manually:**

Bash

`cd /home/sb-dev ./update.sh`

### Wrong Version Showing

**Verify binary version:**

Bash

`/opt/sb-healthcheck/sb-healthcheck --version`

**Check if service restarted:**

Bash

`sudo systemctl status sb-healthcheck # Look for recent restart timestamp`

**Force restart:**

Bash

`sudo systemctl restart sb-healthcheck`

### Script Not Self-Updating

**Check script permissions:**

Bash

`ls -la /home/sb-dev/update.sh # Should be executable: -rwxr-xr-x`

**Manually update script:**

Bash

`cd /home/sb-dev curl -O https://raw.githubusercontent.com/CodeAvolition/sb-healthcheck/main/update.sh chmod +x update.sh`

### GitHub API Rate Limiting

**Symptom:** Updates fail with API errors

**Check rate limit:**

Bash

`curl -s https://api.github.com/rate_limit`

**Why it happens:** GitHub limits unauthenticated API calls to 60/hour per IP

**Solution:** Updates use public releases API which has higher limits. If issues persist, consider adding GitHub token to script.

## Security Considerations

### Why sudo is needed

- Binary installation to `/opt/` requires root
- Service restart requires root
- Log file writing to `/var/log/` requires root

### Permissions

- Update script: Owned by `sb-dev`, executable
- Binary: Owned by root, executable by all
- Log files: Writable by `sb-dev`

### Network security

- Downloads only from official GitHub releases
- HTTPS enforced for all downloads
- No external dependencies beyond GitHub

## Related Documentation

- [Deployment Guide](https://kagi.com/assistant/deployment.md) - Initial setup
- [Development Guide](https://kagi.com/assistant/development.md) - Building and testing
- [Architecture](https://kagi.com/assistant/architecture.md) - System design

# How to connect to the PiZW once im back home:
#### Reconfigure WiFi for different network

Steps to change WiFi before first boot on new network:
1. Insert SD card into laptop
2. Wait for partitions to auto-mount
3. Navigate to boot partition
   `cd /run/media/codea/bootfs`
4. Check if firstrun.sh exists
   `ls -la firstrun.sh`
5. Edit the file
   `sudo nano firstrun.sh`
6. Find the WiFi section (search for SSID)
7. Replace SSID and password with home network credentials
8. Save and exit (Ctrl+X, Y, Enter)
9. Unmount safely
   `sudo umount /run/media/codea/bootfs`
10. Insert SD card into Pi and boot

Note: Modern Raspberry Pi OS uses firstrun.sh for initial config. If file doesn't exist, look for wpa_supplicant.conf in boot partition instead.
