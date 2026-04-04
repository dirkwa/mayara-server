#!/usr/bin/env bash
# Run the spoke viewer, installing dependencies first if needed
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"

if [ ! -d "${DIR}/node_modules" ]; then
    echo "Installing dependencies..."
    npm --prefix "${DIR}" install --no-fund --no-audit --loglevel=error
fi

exec node "${DIR}/spoke_viewer.mjs" "$@"
