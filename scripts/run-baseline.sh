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
# ViaVersion: the bench bot speaks the mineflayer protocol (max 26.1) while
# the server is 26.3 — server-side translation is required for the bot to
# join. Staged whenever the jar was fetched by CI (or manually) — present in
# BOTH A/B legs, so the paired comparison stays fair.
if [ -f "$ROOT/build/viaversion/ViaVersion.jar" ]; then
  cp "$ROOT/build/viaversion/ViaVersion.jar" "$S/plugins/"
fi
# ViaBackwards is REQUIRED for old clients (26.1) on the new server: ViaVersion
# alone only supports newer client versions and the bot gets kicked.
if [ -f "$ROOT/build/viaversion/ViaBackwards.jar" ]; then
  cp "$ROOT/build/viaversion/ViaBackwards.jar" "$S/plugins/"
fi
cp "$PAPER_JAR" "$S/paper.jar"

cat > "$S/eula.txt" <<'EOF'
eula=true
EOF
if [ "${PARITY:-0}" = "1" ]; then
  # PARITY legs: the world generator uses the THE_VOID biome — chunk
  # generation must not create ANY entities. The first parity run failed
  # with chickens interleaved into the entity-id sequence at chunk-gen
  # timing (which differs between legs): each chicken consumed the shared
  # SHARED_RANDOM stream (Entity creation draws it for the UUID) at a
  # DIFFERENT point per leg, desyncing the whole mob-AI decision stream.
  # the_void has zero creature spawns in its chunk-gen step, so every
  # entity in both legs is plugin-spawned on the main thread, after the
  # one-shot SHARED_RANDOM seed. The 4-layer floor (bedrock/dirt/grass)
  # keeps the world walkable for the zombie legs.
  cat > "$S/server.properties" <<'EOF'
level-type=minecraft\:flat
generate-structures=false
online-mode=false
gamemode=creative
force-gamemode=true
view-distance=4
simulation-distance=10
spawn-protection=0
spawn-monsters=false
spawn-animals=false
spawn-npcs=false
max-tick-time=300000
motd=ce-baseline
level-seed=620856203
white-list=false
enforce-whitelist=false
generator-settings={"layers":[{"block":"minecraft:bedrock","height":1},{"block":"minecraft:dirt","height":2},{"block":"minecraft:grass_block","height":1}],"biome":"minecraft:the_void","features":false}
EOF
else
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
# Fixed world seed — the parity A/B (PARITY=1) requires byte-identical
# terrain in both legs; harmless for the perf legs (spawn layout is plugin-
# seeded anyway).
level-seed=620856203
# 26.3 flipped the white-list default to ON (1.21.8 defaulted OFF and the
# bot joined without this line) — the bench bot must never be gated.
white-list=false
enforce-whitelist=false
EOF
fi
cat > "$S/spigot.yml" <<'EOF'
settings:
  timeout-time: 300
  restart-on-crash: false
world-settings:
  default:
    # The 100k-entity fills put ~50-100k entities inside the DEFAULT item
    # tracking range (128 blocks) of the single bench bot: the server
    # streams spawn/metadata for ALL of them every tick (the tracker is
    # 7%+ of vanilla tick CPU) and the mineflayer bot buffers every packet
    # — the runner VM died with 'runner shutdown' (kernel OOM pattern)
    # five runs in a row on exactly the big legs. 48 blocks covers every
    # gameplay-relevant interaction in the bench and applies to BOTH A/B
    # legs equally (paired validity preserved).
    entity-tracking-range:
      players: 48
      animals: 48
      monsters: 48
      misc: 48
      other: 48
EOF

AGENT_ARGS=()
if [ "${MODULE:-off}" = "on" ]; then
  echo "MODULE=on: building CRUSSTY runtime (from source) + c-entities module"
  # 1. mojang-mapped server jar for javac (one-shot paperclip boot, no world)
  # Idempotent: reuse the previously produced mapped jar when present.
  PB="$ROOT/build/paperboot"
  MAPPED=$(find "$PB/versions" -name "paper-26.3.jar" 2>/dev/null | head -1 || true)
  if [ -z "$MAPPED" ]; then
    mkdir -p "$PB"
    cp "$PAPER_JAR" "$PB/paper.jar"
    echo "eula=false" > "$PB/eula.txt"
    (cd "$PB" && timeout 300 java -Xmx2G -jar paper.jar nogui > boot.log 2>&1 || true)
    MAPPED=$(find "$PB/versions" -name "paper-26.3.jar" | head -1)
  fi
  [ -n "$MAPPED" ] && [ -f "$MAPPED" ] || { echo "FATAL: paperclip boot produced no mapped jar"; tail -20 "$PB/boot.log" || true; exit 1; }
  # 2. module cdylib (CRUSSTY path deps live at repo-parent/CRUSSTY)
  (cd "$ROOT" && PAPER_JAR="$MAPPED" cargo build -p centities-module --release) \
    || { echo "FATAL: module build failed"; exit 1; }
  # 3. CRUSSTY runtime from source (release v2.2.5 binary has the hook-name bug)
  (cd "$ROOT/../CRUSSTY/runtime" && cargo build --release -p crussty-runtime) \
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
# Bisect knobs (workflow_dispatch inputs): CE_ITEM_TICK is read via
# System.getenv; the merge knob is a system property in CeHooks — map it.
export CE_ITEM_TICK="${CE_ITEM_TICK:-1}"
if [ "${CE_MERGE:-1}" = "0" ]; then MERGE_PROP="-Dcentities.mergeFast=false"; else MERGE_PROP=""; fi
# L22a native mob AI floor (Rust goal engine): ON by default for every
# MODULE=on leg (baseline -mod runs + ab-paired on-legs).
# SHIPPED DEFAULT = vanilla serverAiStep: the L22a AI engine is opt-in
# (CeHooks MOB_AI_ON_INIT defaults to "0") AND a measured net loss on the
# 26.3 bench (e774d00: mobs-25k 193.2 with the engine vs 129.5 without)
# AND its staged-request RNG protocol diverges level.random from vanilla
# at t=143 (SR/LR probes, run 40771017055). The honest A/B tests what
# ships: vanilla AI unless CE_MOB_AI=1 is explicitly requested.
MOBAI_PROP=""
if [ "${MODULE:-off}" = "on" ] && [ "${CE_MOB_AI:-0}" = "1" ]; then
  MOBAI_PROP="-Dcentities.mobAi=1"
fi
# Same for the L15 GoalSelector floor: shipped = vanilla GoalSelector.
# The CeHooks reader keeps the floor ON unless the property is literally
# "false" — so the shipped path must PASS -Dcentities.aiFloor=false, and
# CE_AI_FLOOR=1 (opt-in) omits the flag.
AIFLOOR_PROP=""
if [ "${MODULE:-off}" = "on" ] && [ "${CE_AI_FLOOR:-0}" != "1" ]; then
  AIFLOOR_PROP="-Dcentities.aiFloor=false"
fi
echo "starting server (heap $XMX) for run=$RUN scale=${SCALE:-0} hoppers=${TILE_HOPPERS:-0} furnaces=${TILE_FURNACES:-0} module=${MODULE:-off} itemTick=$CE_ITEM_TICK merge=${CE_MERGE:-1} mobAi=${MOBAI_PROP:+on}"
java -Xms"${XMS:-2G}" -Xmx"$XMX" -XX:+UseG1GC -XX:MaxGCPauseMillis=50 -XX:+ExitOnOutOfMemoryError \
  -XX:NativeMemoryTracking=summary \
  ${MERGE_PROP} \
  ${MOBAI_PROP} \
  ${AIFLOOR_PROP} \
  ${EXTRA_JVM:-} \
  "${AGENT_ARGS[@]}" \
  -XX:StartFlightRecording="filename=$R/rec.jfr,settings=profile,dumponexit=true,maxsize=256m" \
  -jar paper.jar nogui > "$R/server.log" 2>&1 &
JPID=$!

# Forensics watchdog: a runner death (exit 143) kills the job — artifacts
# never upload — but ALREADY-STREAMED job-log lines survive. Stream the
# module telemetry + per-process RSS into the job log every 30s so the
# exact death phase (fill/measure), the module's last words and the RSS
# of java/node are captured no matter how the leg dies.
(
  PHDONE=0
  for i in $(seq 1 120); do
    sleep 10
    kill -0 "$JPID" 2>/dev/null || break
    # NOTE: the subshell inherits `set -e` — a bare `[ $i -gt 18 ]` that
    # evaluates false KILLS the watchdog after the first sample (observed:
    # every dead-leg log had exactly one WD line). Use an if.
    if [ "$i" -gt 18 ]; then sleep 20; fi
    JRSS=$(ps -o rss= -p "$JPID" 2>/dev/null | awk '{printf "%.0fMB", $1/1024}' || true)
    NRSS=$(ps -o rss= -C node 2>/dev/null | awk 'NR==1{printf "%.0fMB", $1/1024}' || true)
    SYSMEM=$(free -m 2>/dev/null | awk '/Mem:/{printf "%s/%sMB", $3, $2}' || true)
    echo "WD[$i] rss java=$JRSS node=${NRSS:-0} mem=$SYSMEM"
    # jcmd attaches via the VM's attach mechanism — it can STALL for minutes
    # on a busy JVM (observed: the watchdog hung inside jcmd and the NMT
    # sample never landed). Hard-cap it. EVERY pipeline in this loop needs
    # `|| true`: pipefail + grep-no-match kills the subshell silently.
    timeout 5 jcmd "$JPID" VM.native_memory summary 2>/dev/null | grep -E "Total: reserved=" | head -1 || true
    grep -aE "\[c-entities\] (ARMED|NOT armed|FAILED|item-flush|verifier snapshot|precomputed|patched)|OutOfMemory|VerifyError|Watchdog" "$R/server.log" 2>/dev/null | tail -3 || true
    # Delta-tail the parity stream into the job log: the per-tick state hash
    # (PH) diff across legs is the FIRST-DIVERGENCE locator, and the
    # parity-results artifact is ~1.8GB — pulling it after the fact timed
    # out at 600s and the analysis never ran (observed 13:08 run). Lines
    # streamed here survive any runner death.
    TOTAL=$(wc -l < "$R/server.log" 2>/dev/null || echo 0)
    if [ "$TOTAL" -gt "$PHDONE" ]; then
      tail -n +$((PHDONE + 1)) "$R/server.log" 2>/dev/null \
        | grep -aE "PH t=|PD t=|SR t=|LR t=|MS t=|PTRACE |PARITY-SNAP|measure window|hooks@|entropy probe" || true
      PHDONE=$TOTAL
    fi
  done
) &
WDPID=$!

# mineflayer is installed into tools/node_modules (both on CI and locally);
# bot.js resolves modules from its own directory upward, which never reaches
# tools/ — pass NODE_PATH explicitly (the require() fallback path).
cd "$ROOT/tools" 2>/dev/null || cd "$ROOT"
BOTPID=0
if [ "${PARITY:-0}" != "1" ]; then
  # Belt against the runner-OOM pattern: cap the bot's heap so a packet
  # storm degrades the BOT (graceful FATAL) instead of the whole VM.
  NODE_HEAP="--max-old-space-size=1536"
  NODE_PATH="$ROOT/tools/node_modules${NODE_PATH:+:$NODE_PATH}" node $NODE_HEAP "$ROOT/scripts/bot.js" &
  BOTPID=$!
else
  echo "PARITY=1: bot skipped (player motion is wallclock-random — it would break determinism)"
fi

JSON_OK=0
for i in $(seq 1 "$RUN_TIMEOUT"); do
  if [ -f "$S/baseline.json" ]; then JSON_OK=1; break; fi
  if ! kill -0 "$JPID" 2>/dev/null; then break; fi
  sleep 2
done

if [ "$JSON_OK" != "1" ]; then
  echo "FATAL: no baseline.json (timeout or crash)"
  # grab the JVM fatal report BEFORE exiting — the only forensic for a
  # GC-barrier SIGSEGV (the corrupting thread's frames live there)
  cp "$S"/hs_err_pid*.log "$R/" 2>/dev/null || true
  cp "$S"/hs_err_pid*.jfr "$R/" 2>/dev/null || true
  tail -n 80 "$R/server.log" || true
  kill -TERM "$JPID" 2>/dev/null || true
  exit 1
fi
echo "baseline.json present — waiting for graceful JVM exit (JFR dump)"
for i in $(seq 1 90); do kill -0 "$JPID" 2>/dev/null || break; sleep 2; done
kill -TERM "$JPID" 2>/dev/null || true
sleep 5
[ "$BOTPID" != 0 ] && kill "$BOTPID" 2>/dev/null || true
kill "$WDPID" 2>/dev/null || true
kill -9 "$JPID" 2>/dev/null || true

cp "$S/baseline.json" "$R/"
cp "$S/logs/latest.log" "$R/" 2>/dev/null || true
# Parity snapshots: copy every parity-*.json the run produced (the A/B
# gate compares them across the off/on legs).
cp "$S"/parity-*.json "$R/" 2>/dev/null || true
# Parity per-tick trace (PARITY_TRACE_ID): the line diff across the legs
# names the diverging TICK and FIELD directly.
cp "$S"/parity-trace*.jsonl "$R/" 2>/dev/null || true
# Crash forensics: JVM fatal-error reports land next to the server dir —
# without them a SIGSEGV in a GC barrier is undebuggable from CI alone
# (the hs_err stack is the ONLY place the corrupting thread's frames live).
cp "$S"/hs_err_pid*.log "$R/" 2>/dev/null || true
cp "$S"/hs_err_pid*.jfr "$R/" 2>/dev/null || true
cp "$S"/core.* "$R/" 2>/dev/null || true || true
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
