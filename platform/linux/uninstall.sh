#!/bin/sh

error_exit() {
    echo "Failed to uninstall aquote"
    exit 1
}

# Stop, disable, and delete scheduled job
systemctl stop aquote-fetch.timer || error_exit
systemctl disable aquote-fetch.timer || error_exit
rm /etc/systemd/system/aquote-fetch.timer || error_exit
rm /etc/systemd/system/aquote-fetch.service || error_exit
systemctl daemon-reload || error_exit

# Delete binary
rm /usr/local/bin/aquote || error_exit

# Delete config
rm -rf /etc/aquote || error_exit

echo "Successfully uninstalled aquote"

echo "Remove \"aquote show\" from your .bashrc, .zshrc, fish_greeting.fish, etc."
