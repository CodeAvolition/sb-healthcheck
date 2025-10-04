#!/bin/bash

LOG_FILE="/var/log/sb-healthcheck-updates.log"
MAX_AGE_DAYS=2

# Only rotate if log exists and is older than 2 days
if [ -f "$LOG_FILE" ]; then
    find "$LOG_FILE" -mtime +$MAX_AGE_DAYS -exec truncate -s 0 {} \;
fi
