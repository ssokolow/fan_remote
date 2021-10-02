#!/bin/sh

cd "$(dirname "$(readlink -f "$0")")" || exit

cp fan_remote.service /etc/systemd/system/
cp target/release/fan_remote /usr/local/bin/
systemctl enable --now fan_remote.service
