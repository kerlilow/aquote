#!/bin/sh

# Install binary
cp aquote /usr/local/bin/

# Initialize config
mkdir /etc/aquote
cp config/default.toml /etc/aquote/config.toml

# Add, enable, and start scheduled job
cp systemd/* /etc/systemd/system/
systemctl daemon-reload
systemctl enable aquote-fetch.timer
systemctl start aquote-fetch.timer
