#!/usr/bin/env bash
# One full baseline run: server dir -> boot Paper + bot -> wait for results.
# Config comes from env (SCALE, PCT_ITEMS, ..., RUN_NAME, RUN_TIMEOUT).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
RUN="${RUN_NAME:-run}"
S="$ROOT/bench/$RUN/server"
R="$ROOT/bench/$RUN/results"
PAPER_JAR="${PAPER_JAR:-$ROOT/server/paper.jar}"
RUN_TIMEOUT="${RUN_TIMEOUT:-1000}"

[ -f "$PAPER_JAR" ] || { echo "FATAL: paper jar not found at $PAPER_JAR"; exit 1; }
[ -f "$ROOT/build/ce-baseline.jar" ] || { echo "FATAL: plugin not built"; exit 1; }

mkdir -p "$S/plugins" "$R"
cp "$ROOT/build/ce-baseline.jar" "$S/plugins/"
cp "$PAPER_JAR" "$S/paper.jar"

cat > "$S/eula.txt" <<'EOF'
eula=true
EOF
cat > "$S/server.properties" <<'EOF'
level-type=minecraft:flat
generate-structures=false
online-mode=false
gamemode=creative
force-gamemode=true
view-distance=4
simulation-distance=10
spawn-protection=0
spawn-monsters=false
max-tick-time=300000
motd=ce-baseline
EOF
cat > "$S/spigot.yml" <<'EOF'
settings:
  timeout-time: 300
  restart-on-crash: false
EOF

AGENT_ARGS=()
if [ "${MODULE:-off}" = "on" ]; then
  echo "MODULE=on: building CRUSSTY runtime (from source) + c-entities module"
  # 1. mojang-mapped server jar for javac (one-shot paperclip boot, no world)
  PB="$ROOT/build/paperboot"
  mkdir -p "$PB"
  cp "$PAPER_JAR" "$PB/paper.jar"
  echo "eula=false" > "$PB/eula.txt"
  (cd "$PB" && timeout 300 java -Xmx2G -jar paper.jar nogui > boot.log 2>&1 || true)
  MAPPED=$(find "$PB/versions" -name "paper-1.21.8.jar" | head -1)
  [ -n "$MAPPED" ] && [ -f "$MAPPED" ] || { echo "FATAL: paperclip boot produced no mapped jar"; tail -20 "$PB/boot.log" || true; exit 1; }
  # 2. module cdylib (CRUSSTY path deps live at repo-parent/CRUSSTY)
  (cd "$ROOT" && PAPER_JAR="$MAPPED" cargo build -p centities-module --release) \
    || { echo "FATAL: module build failed"; exit 1; }
  # 3. CRUSSTY runtime from source (release v2.2.5 binary has the hook-name bug)
  (cd "$ROOT/../CRUSSTY" && cargo build --release -p crussty-runtime) \
    || { echo "FATAL: runtime build failed"; exit 1; }
  # 4. stage into the bench server
  mkdir -p "$S/modules/entities"
  cp "$ROOT/target/release/libcentities_module.so" "$S/modules/entities/"
  cp "$ROOT/module.json" "$S/modules/entities/"
  cp "$ROOT/../CRUSSTY/runtime/target/release/libcrussty_runtime.so" "$S/"
  AGENT_ARGS=(-agentpath:"$S/libcrussty_runtime.so=modules=$S/modules;versions=$S;kernel=paper.jar")
fi

cd "$S"
XMX="${XMX:-8G}"
echo "starting server (heap $XMX) for run=$RUN scale=${SCALE:-0} hoppers=${TILE_HOPPERS:-0} furnaces=${TILE_FURNACES:-0} module=${MODULE:-off}"
java -Xms2G -Xmx"$XMX" -XX:+UseG1GC -XX:MaxGCPauseMillis=50 \
  "${AGENT_ARGS[@]}" \
  -XX:StartFlightRecording="filename=$R/rec.jfr,settings=profile,dumponexit=true" \
  -jar paper.jar nogui > "$R/server.log" 2>&1 &
JPID=$!

cd "$ROOT/tools" 2>/dev/null || cd "$ROOT"
node "$ROOT/scripts/bot.js" &
BOTPID=$!

JSON_OK=0
for i in $(seq 1 "$RUN_TIMEOUT"); do
  if [ -f "$S/baseline.json" ]; then JSON_OK=1; break; fi
  if ! kill -0 "$JPID" 2>/dev/null; then break; fi
  sleep 2
done

if [ "$JSON_OK" != "1" ]; then
  echo "FATAL: no baseline.json (timeout or crash)"
  tail -n 80 "$R/server.log" || true
  kill -TERM "$JPID" 2>/dev/null || true
  exit 1
fi
echo "baseline.json present — waiting for graceful JVM exit (JFR dump)"
for i in $(seq 1 90); do kill -0 "$JPID" 2>/dev/null || break; sleep 2; done
kill -TERM "$JPID" 2>/dev/null || true
sleep 5
kill "$BOTPID" 2>/dev/null || true
kill -9 "$JPID" 2>/dev/null || true

cp "$S/baseline.json" "$R/"
cp "$S/logs/latest.log" "$R/" 2>/dev/null || true
if [ "${MODULE:-off}" = "on" ]; then
  grep -q "\[c-entities\] ARMED" "$R/server.log" \
    || { echo "FATAL: module did not ARM — A/B is meaningless"; grep "\[c-entities\]" "$R/server.log" | head -20; exit 1; }
  echo "module ARMED confirmed"
fi
python3 "$ROOT/scripts/jfr_top.py" "$R/rec.jfr" > "$R/hotspots.md" 2>/dev/null \
  || echo "(jfr parse failed)" > "$R/hotspots.md"
python3 "$ROOT/scripts/make-report.py" "$R" || true
[ -f "${GITHUB_STEP_SUMMARY:-}" ] && cat "$R/REPORT.md" >> "$GITHUB_STEP_SUMMARY" || true
echo "OK: results in $R"
