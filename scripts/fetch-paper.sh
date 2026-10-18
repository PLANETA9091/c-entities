#!/usr/bin/env bash
# Download Paper server jar via the official Fill API (v3).
# Usage: fetch-paper.sh [mc-version] [out-jar]
set -euo pipefail
MC="${1:-1.21.8}"
OUT="${2:-server.jar}"
UA="c-entities-baseline/0.1 (github.com/PLANETA9091/c-entities)"

JSON=$(curl -fsS -H "User-Agent: $UA" "https://fill.papermc.io/v3/projects/paper/versions/$MC/builds?channel=STABLE")
URL=$(echo "$JSON" | python3 -c "import sys,json;print(json.load(sys.stdin)[0]['downloads']['server:default']['url'])")
mkdir -p "$(dirname "$OUT")"
echo "downloading $MC -> $URL"
curl -fsSL -H "User-Agent: $UA" -o "$OUT" "$URL"
echo "sha256: $(sha256sum "$OUT" | cut -d' ' -f1)"
echo "done: $OUT ($(du -h "$OUT" | cut -f1))"
