#!/usr/bin/env bash
# A/B benchmark: Paper 1.21.8 with ce-baseline harness, module ON vs OFF.
# Usage: run_ab.sh <scenario> <on|off> [results.csv]
#
# The ce-baseline plugin starts itself (45s failsafe), fills the world,
# warms up (runs the CE_DIFF differential test when CE_DIFF=1), measures
# MEASURE_TICKS, writes baseline.json, then shuts the server down. We poll
# for baseline.json and kill the JVM as soon as it appears (no graceful
# wait: moonrise pools would add minutes of pointless drain time).
set -u

SCEN="${1:?scenario: control|items-100k|mobs-25k|mixed-150k}"
MODE="${2:?mode: on|off}"
CSV="${3:-/home/z/my-project/work/ab/results.csv}"
REPO=/home/z/my-project/work/c-entities
CRUSSTY=/home/z/my-project/work/CRUSSTY
JDK=/home/z/tools/jdk-21.0.13+9

case "$SCEN" in
    control)     SCALE=0;      PCT_ITEMS=1.0; PCT_STANDS=0.0; TILES=0; WARM=100; MEAS=200 ;;
    items-100k)  SCALE=100000; PCT_ITEMS=1.0; PCT_STANDS=0.0; TILES=0; WARM=100; MEAS=200 ;;
    mobs-25k)    SCALE=25000;  PCT_ITEMS=0.0; PCT_STANDS=0.0; TILES=0; WARM=100; MEAS=200 ;;
    mixed-150k)  SCALE=150000; PCT_ITEMS=0.7; PCT_STANDS=0.2; TILES=0; WARM=40;  MEAS=120 ;;
    *) echo "unknown scenario $SCEN" >&2; exit 1 ;;
esac

RUN_DIR=/home/z/my-project/work/ab/$SCEN-$MODE
PORT=$(( 25600 + RANDOM % 200 ))
rm -rf "$RUN_DIR"
mkdir -p "$RUN_DIR/plugins"
cp "$REPO/baseline/plugin/target/ce-baseline-0.1.0.jar" "$RUN_DIR/plugins/"
# paper jar FIRST (its patched classes must win), then every library jar
SERVER_CP="$REPO/server/versions/1.21.8/paper-1.21.8.jar:$(find "$REPO/server/libraries" -name '*.jar' | sort | tr '\n' ':')"

cat > "$RUN_DIR/eula.txt" <<EOF
eula=true
EOF
cat > "$RUN_DIR/server.properties" <<EOF
server-port=$PORT
level-type=minecraft\:flat
generate-structures=false
online-mode=false
spawn-protection=0
view-distance=4
simulation-distance=4
spawn-monsters=false
motd=ab
EOF

# world template: reuse a previously generated flat world to skip worldgen
TPL=/home/z/my-project/work/ab/world-template
if [ -d "$TPL" ]; then
    cp -r "$TPL" "$RUN_DIR/world"
fi

AGENT_ARGS=()
if [ "$MODE" = "on" ]; then
    # the CRUSSTY runtime is the JVMTI agent; it dlopens our module and
    # calls cplugin_init (the module .so itself has no Agent_OnLoad)
    AGENT_ARGS=("-agentpath:$CRUSSTY/libcrussty_runtime.so=modules=$CRUSSTY/modules;versions=$CRUSSTY/versions;kernel=paper.jar")
fi

echo "[ab] $SCEN mode=$MODE port=$PORT dir=$RUN_DIR"
cd "$RUN_DIR"
env RUN_NAME="$SCEN-$MODE" SCALE="$SCALE" PCT_ITEMS="$PCT_ITEMS" PCT_STANDS="$PCT_STANDS" \
    TILE_HOPPERS=0 TILE_FURNACES=0 MEASURE_TICKS=$MEAS WARMUP_TICKS=$WARM \
    FILL_TICKS=400 CE_DIFF=1 \
    $JDK/bin/java \
    -Xms1G -Xmx2G -XX:+UseG1GC \
    "${AGENT_ARGS[@]}" \
    -cp "$SERVER_CP" \
    org.bukkit.craftbukkit.Main nogui > server.out 2>&1 < /dev/null &
JAVA_PID=$!

START=$SECONDS
DEADLINE=$(( SECONDS + 570 ))
RESULT=""
while :; do
    if [ -f "$RUN_DIR/baseline.json" ]; then RESULT=ok; break; fi
    if ! kill -0 "$JAVA_PID" 2>/dev/null; then RESULT=dead; break; fi
    if [ "$SECONDS" -ge "$DEADLINE" ]; then RESULT=timeout; break; fi
    sleep 5
done

if [ "$RESULT" = "ok" ]; then
    sleep 1
    kill "$JAVA_PID" 2>/dev/null
    wait "$JAVA_PID" 2>/dev/null
else
    echo "[ab] FAIL ($RESULT) after $((SECONDS-START))s; tail:"
    kill -9 "$JAVA_PID" 2>/dev/null
    tail -c 2000 "$RUN_DIR/server.out" | tr '\r' '\n' | grep -aE "INFO|ERROR|Exception" | tail -5
    exit 1
fi

# keep a clean world template after the first successful boot
if [ ! -d "$TPL" ]; then
    rm -rf "$RUN_DIR/world/$SCEN-nogui-backup" 2>/dev/null || true
    mkdir -p /home/z/my-project/work/ab
    cp -r "$RUN_DIR/world" "$TPL" 2>/dev/null && echo "[ab] world template saved"
fi

python3 - "$RUN_DIR/baseline.json" "$SCEN" "$MODE" "$CSV" <<'PYEOF'
import json, sys, os
j = json.load(open(sys.argv[1]))
scen, mode, csv = sys.argv[2], sys.argv[3], sys.argv[4]
ms = j["mspt"]
cd = j.get("ce_diff", {})
cm = j.get("ce_module", {})
row = f"{scen},{mode},{ms['mean']:.3f},{ms['p50']:.3f},{ms['p99']:.3f},{ms['max']:.3f}," \
      f"{cd.get('checks',0)},{cd.get('mismatches',0)},{cm.get('active',False)},{cm.get('fast',0)},{cm.get('mirror',0)}"
new = not os.path.exists(csv)
with open(csv, "a") as f:
    if new:
        f.write("scenario,mode,mean_ms,p50_ms,p99_ms,max_ms,diff_checks,diff_mismatch,armed,fast,mirror\n")
    f.write(row + "\n")
print("[ab] RESULT", row)
PYEOF
echo "[ab] done $SCEN-$MODE"
exit 0
