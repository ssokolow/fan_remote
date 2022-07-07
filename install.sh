#!/bin/sh

cd "$(dirname "$(readlink -f "$0")")" || exit

is_installed() {
    type "$1" 1>/dev/null 2>&1
    return $?
}

# Ensure an appropriately minimally-featured release build
# if we're not root
if [ "$(id -u)" != 0 ]; then
    cargo build --release --no-default-features
    exec sudo "$0" "$@"
fi

if ! is_installed br; then
    apt-get install bottlerocket
fi

# Copy the server binary into place
systemctl disable --now fan_remote.service
cp target/release/fan_remote /usr/local/bin/

# Poke a hole in the firewall
cp fan_remote.ufw /etc/ufw/applications.d/fan_remote
ufw allow fan_remote

# Set up the systemd service, enable it, and start it
cp fan_remote.socket fan_remote.service /etc/systemd/system/
systemctl enable --now fan_remote.socket
