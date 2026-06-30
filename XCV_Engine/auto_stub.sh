#!/bin/bash
MISSING_FILE=$1
DIR=$(dirname "$MISSING_FILE")
mkdir -p "$DIR"
touch "$MISSING_FILE"
echo "[+] Stub created: $MISSING_FILE"
