#! /bin/bash
set -e #Exit immediately on any error

echo "building program..."
cargo build --release

# Linux build
EX_PATH="/usr/local/bin/beep"
sudo cp target/release/beep $EX_PATH 
echo "Copied the executable to $EX_PATH"

echo "RUN: beep --help"
