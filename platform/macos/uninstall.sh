#!/bin/sh

error_exit() {
    echo "Failed to uninstall aquote"
    exit 1
}

# Unload and remove scheduled job
launchctl unload -w /Library/LaunchDaemons/me.kerlilow.aquote.fetch.plist || error_exit
rm /Library/LaunchDaemons/me.kerlilow.aquote.fetch.plist || error_exit

# Delete binary
rm /usr/local/bin/aquote || error_exit

# Delete config
rm -rf /etc/aquote || error_exit

echo "Successfully uninstalled aquote"

echo "Remove \"aquote show\" from your .bashrc, .zshrc, fish_greeting.fish, etc."
