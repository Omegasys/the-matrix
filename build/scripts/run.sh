#!/bin/bash

echo "[RUN] Launching MatrixNet..."

cd "$(dirname "$0")/../.."

cargo run

if [ $? -ne 0 ]; then
    echo "[RUN] Execution failed"
    exit 1
fi
