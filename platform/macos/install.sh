#!/bin/sh

error_exit() {
    echo "Failed to install aquote"
    exit 1
}

# Initialize config
mkdir -p /etc/aquote || error_exit
cp config/default.toml /etc/aquote/config.toml || error_exit

# Install binary
cp aquote /usr/local/bin/ || error_exit

# Add and load scheduled job
cp launchd/me.kerlilow.aquote.fetch.plist /Library/LaunchDaemons/ || error_exit
launchctl load -w /Library/LaunchDaemons/me.kerlilow.aquote.fetch.plist || error_exit

echo "Successfully installed aquote"

echo "Add \"aquote show\" to your .bashrc, .zshrc, fish_greeting.fish, etc. \
to display the quote of the day when you start your shell"
