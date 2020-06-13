#!/bin/sh

DEVICE_ID=2
PORT=23734
TARGET_BIN=./target/release/fan_remote

cd "$(dirname "$(readlink -f "$0")")"

if [ ! -f "$TARGET_BIN" ]; then
    cargo build --release
    strip "$TARGET_BIN"
fi

echo "Running on port $PORT for device ID $DEVICE_ID..."
firejail --profile=fan_remote.firejail --private=./target/release -- \
    ./fan_remote -p$PORT -F$DEVICE_ID "$@"
