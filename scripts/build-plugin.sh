#!/usr/bin/env bash
# Compile the baseline plugin (Maven, paper-api 1.21.8 from repo.papermc.io).
# Requires: JDK 21 + Maven (both available via setup-java/setup-maven on CI).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
mkdir -p "$ROOT/build"

mvn -q -B -f "$ROOT/baseline/plugin/pom.xml" package -DskipTests
cp "$ROOT/baseline/plugin/target/ce-baseline.jar" "$ROOT/build/ce-baseline.jar"
echo "built: $ROOT/build/ce-baseline.jar"
