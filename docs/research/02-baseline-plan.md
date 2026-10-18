# Baseline benchmark plan: "what happens with nothing"

## Goal

Capture an **honest profile of untouched Paper 1.21.8** under heavy entity
load: how much it costs in MSPT/TPS/RAM and **which exact JVM methods burn
the time**. These numbers drive the c-entities design. No optimization
until this data exists.

## Where it runs

GitHub Actions (`ubuntu-latest`, 4 vCPU, 16 GB RAM) — workflow
`baseline.yml`. Each run = one runner, matrix jobs run in parallel.
Artifacts: `baseline.json` (raw measurements), `rec.jfr` (Java Flight
Recorder), `hotspots.md` (top frames), `REPORT.md` (per-run summary).
The publish job commits the combined report to `reports/baseline/`.

## Methodology (important decisions)

1. **Superflat world** (`level-type=minecraft:flat`), view-distance=4,
   structures off, randomTickSpeed=0, doMobSpawning=false, clear weather —
   so we measure entities, not generation/light/random ticks.
2. **Fake player** (`baseline_bot`, mineflayer, offline mode, creative) —
   without a player, Paper EAR sends all mobs into `inactiveTick()` and
   the benchmark would lie. The bot stands at spawn; entities fill around
   it (part of the mobs within the 28-block activation radius for a fair
   AI workload, the rest outside — like real farms).
3. **Plugin chunk tickets** (level 31 = entity ticking) over the whole
   region — otherwise chunks without players don't tick entities.
4. **Gradual fill** (batches spread over N ticks) — no startup death.
5. **Measurement**: `ServerTickEndEvent.getTickDuration()` (ms/tick) for
   600 ticks after warm-up; percentiles p50/p90/p99/max; entity counters
   every 100 ticks (merge shrinkage is visible); heap before/after.
6. **JFR** (`-XX:StartFlightRecording=settings=profile,dumponexit=true`) —
   top self-time frames are extracted from the recording → "what loads
   what" inside the JVM. JFR overhead ~2-5% — identical for all runs,
   affects absolutes only, not comparisons.
7. The server writes `baseline.json` itself, logs `BASELINE-DONE`, then
   shuts down (`Bukkit.shutdown()`) — so JFR dumps cleanly on exit.

## Run matrix

| Run         | Items  | Stands | Zombies | Block entities           | Purpose |
|-------------|-------:|-------:|--------:|--------------------------|---------|
| control     | 0      | 0      | 0       | 0                        | world floor: chunks, light, tick loop |
| items-25k   | 25 000 | 0      | 0       | 0                        | item cost (gravity+merge) |
| items-100k  | 100 000| 0      | 0       | 0                        | item scaling |
| mobs-10k    | 0      | 0      | 10 000  | 0                        | AI/pathfinding/collision cost |
| mobs-25k    | 0      | 0      | 25 000  | 0                        | mob scaling |
| mixed-50k   | 35k    | 10k    | 5k      | 0                        | realistic mix |
| mixed-150k  | 105k   | 30k    | 15k     | 0                        | 100k+ stress |
| tiles-25k   | 0      | 0      | 0       | 15k hoppers + 10k furnaces | block-entity cost |
| tiles-100k  | 0      | 0      | 0       | 60k hoppers + 40k furnaces | block-entity stress |

Item composition: 50% with pickupDelay=32767 (permanent, no merge),
50% mergeable (real merge scans). Counter shrinkage = merge at work.
Marker armor stands — a cheap per-entity tick floor.
Zombies: persistence + iron helmet (no daylight burning),
Paper `canMobPickup=false`.

## What we get and how it's used

1. **MSPT-vs-scale curve** — the price of the base and of each load class.
2. **Top hot methods** — a ranked list of optimization targets
   (confirms/refutes the hypotheses of doc 01 §6).
3. **Vanilla RAM profile** at 100k+ — the reference for the Rust mirror.
4. The control point for future A/B: same pipeline + CRUSSTY module.

No "optimization" conclusions are made before these numbers exist.
