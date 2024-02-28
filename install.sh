#!/bin/bash

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)

if ! cargo build --release; then
	echo -e "Failed to build gptcli"
	exit 1
else
	echo -e "Built gptcli binary"
fi

if [ -f "$SCRIPT_DIR/target/release/gptcli" ]; then
	printf "Installing chat script..."
	sudo cp "$SCRIPT_DIR/target/release/gptcli" "/usr/local/bin/chat"
	printf "Installed\n"
else
	echo -e "Failed to find built exectuable"
fi
