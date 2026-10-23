#!/usr/bin/env bash
# Rebuild the e2e stand (server/) from scratch — restores after a machine
# restart / FS rollback. Idempotent: skips steps whose outputs exist.
#
# Usage: bash scripts/make-stand.sh
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CRUSSTY="${CRUSSTY:-$ROOT/../CRUSSTY}"
S="$ROOT/server"
JDK25="${JDK25:-$HOME/tools/jdk-25.0.5+8}"

mkdir -p "$S/versions" "$S/modules/entities" "$S/launcher"

# 1. paperclip jar (the bench/stand kernel)
if [ ! -f "$S/versions/paper-26.3.jar" ]; then
    echo "[stand] downloading Paper 26.3 (paperclip)"
    bash "$ROOT/scripts/fetch-paper.sh" 26.3 "$S/versions/paperclip-26.3.jar"
    JAVA_HOME="$JDK25" PATH="$JDK25/bin:$PATH" bash -c '
        cd "$(dirname "$1")" && echo "eula=false" > eula.txt && timeout 420 java -Xmx2G -jar "$(basename "$1")" nogui > boot.log 2>&1 || true
    ' _ "$S/versions/paperclip-26.3.jar"
fi

# 2. runtime + launcher from the CRUSSTY clone (release builds)
echo "[stand] building CRUSSTY runtime + launcher"
cargo build --manifest-path "$CRUSSTY/runtime/Cargo.toml" --release
cp "$CRUSSTY/runtime/target/release/libcrussty_runtime.so" "$S/"
mkdir -p "$CRUSSTY/launcher/out"
JAVA_HOME="$JDK25" PATH="$JDK25/bin:$PATH" javac -d "$CRUSSTY/launcher/out" \
    "$CRUSSTY/launcher/src/main/java/dev/dist/launcher/Main.java"
JAVA_HOME="$JDK25" PATH="$JDK25/bin:$PATH" jar cfe "$CRUSSTY/launcher/launcher.jar" \
    dev.dist.launcher.Main -C "$CRUSSTY/launcher/out" .
cp "$CRUSSTY/launcher/launcher.jar" "$S/launcher/"

# 3. module cdylib (paper jar for javac comes from $ROOT/server/paper.jar)
echo "[stand] building c-entities module"
JAVA_HOME="$JDK25" PATH="$JDK25/bin:$PATH" \
    cargo build --manifest-path "$ROOT/Cargo.toml" -p centities-module --release
cp "$ROOT/target/release/libcentities_module.so" "$S/modules/entities/"
cp "$ROOT/module.json" "$S/modules/entities/"

# 4. server config
[ -f "$S/eula.txt" ] || printf 'eula=true\n' > "$S/eula.txt"
cat > "$S/server.properties" <<'EOF'
level-type=minecraft\:flat
online-mode=false
server-port=25585
view-distance=4
simulation-distance=4
EOF

echo "[stand] ready: $S (boot: cd $S && java -jar launcher/launcher.jar --port 25585)"
