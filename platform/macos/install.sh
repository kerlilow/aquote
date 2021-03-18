#!/bin/sh

# Install binary
cp aquote /usr/local/bin/

# Initialize config
mkdir /etc/aquote
cp config/default.toml /etc/aquote/config.toml

# Add scheduled job
cp launchd/me.kerlilow.aquote.fetch.plist /Library/LaunchDaemons/
