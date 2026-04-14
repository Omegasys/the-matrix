#!/bin/bash

echo "[BUILD] Starting MatrixNet build..."

# Ensure we're in project root
cd "$(dirname "$0")/../.."

# Build using Cargo
cargo build

if [ $? -ne 0 ]; then
    echo "[BUILD] Failed"
    exit 1
fi

echo "[BUILD] Success"
