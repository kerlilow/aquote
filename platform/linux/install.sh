#!/bin/sh

error_exit() {
    echo "Failed to install aquote"
    exit 1
}

# # Initialize config
mkdir -p /etc/aquote || error_exit
cp config/default.toml /etc/aquote/config.toml || error_exit

# # Install binary
cp aquote /usr/local/bin/ || error_exit

# # Add, enable, and start scheduled job
cp systemd/* /etc/systemd/system/ || error_exit
systemctl daemon-reload || error_exit
systemctl enable aquote-fetch.timer || error_exit
systemctl start aquote-fetch.timer || error_exit

echo "Successfully installed aquote"

echo "Add \"aquote show\" to your .bashrc, .zshrc, fish_greeting.fish, etc. \
to display the quote of the day when you start your shell"
