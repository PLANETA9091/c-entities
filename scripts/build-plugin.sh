#!/usr/bin/env bash
# Compile the baseline plugin against paper-api (no Maven/Gradle).
# Requires: JDK 21 (javac), curl, python3.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MC="1.21.8"
BUILD="$ROOT/build"
UA="c-entities-baseline/0.1 (github.com/PLANETA9091/c-entities)"
mkdir -p "$BUILD/classes"

if [ ! -f "$BUILD/paper-api.jar" ]; then
  META="https://repo.papermc.io/repository/maven-public/io/papermc/paper/paper-api/${MC}-R0.1-SNAPSHOT/maven-metadata.xml"
  SNAP=$(curl -fsSL "$META" | tr -d ' \n' | grep -oPm1 '(?<=<value>)[^<]+' | tail -1)
  echo "paper-api snapshot: $SNAP"
  curl -fsSL -o "$BUILD/paper-api.jar" \
    "https://repo.papermc.io/repository/maven-public/io/papermc/paper/paper-api/${MC}-R0.1-SNAPSHOT/paper-api-${SNAP}.jar"
fi

javac --release 21 -Xlint:-options -cp "$BUILD/paper-api.jar" -d "$BUILD/classes" \
  "$ROOT/baseline/plugin/BaselinePlugin.java"

cd "$BUILD/classes"
jar --create --file "$BUILD/ce-baseline.jar" . -C "$ROOT/baseline/plugin" plugin.yml
echo "built: $BUILD/ce-baseline.jar"
