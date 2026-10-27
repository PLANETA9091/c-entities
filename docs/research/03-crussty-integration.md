# c-entities integration into Paper 1.21.8 via the CRUSSTY platform

## Base

- **Server**: Paper 1.21.8 (mojang-mapped runtime since 1.20.5), **Java 21**
  (matches CRUSSTY's own e2e: java-21-openjdk-amd64).
- **Platform**: CRUSSTY — native Rust plugins `c-*` injected into the JVM,
  bytecode weaving via ASM. Verified APIs (cplug-sdk):
  - `cplug_sdk::weave::replace_body(...)` — rewrite a method body
    (a correct StackMapTable frame is mandatory on Java 21+);
  - `cplug_sdk::weave::redirect_calls(...)` — redirect call sites;
  - `cplug_sdk::run_on_main_thread(...)` — execute on the server main thread;
  - telemetry registry (`TelemetryRegistry`) — profile metric export.

## Injection points in Paper 1.21.8 (from the sources, see doc 01)

| # | Point | Purpose |
|---|-------|---------|
| 1 | `ServerLevel.tickNonPassenger(Entity)` | the main hub: decide "full vanilla tick vs Rust mirror tick" |
| 2 | `ServerLevel.tickPassenger(Entity, Entity, boolean)` | passengers of mirrored entities |
| 3 | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level)` | remove the O(N) scan over mirrored entities |
| 4 | `ActivationRange.checkIfActive(Entity)` | short-circuit for mirrored entities |
| 5 | `Level.tickBlockEntities()` | route block entities into the Rust scheduler |
| 6 | (opt.) `ItemEntity.tick` / merge | item physics mirror |

Kill switch: any weaving failure → the module runs in telemetry-only mode,
entities tick vanilla. We never break the server.

## What the mirror does

The Rust core keeps an SoA copy of positions/velocities/states. On tick:

- entities that are **provably static** (on_ground, zero velocity, no goal,
  no player nearby, calm for ≥ N ticks, not in water/fire) are skipped —
  vanilla itself would give them `inactiveTick` and EAR grants immunities;
  we never change observable behavior;
- **everything else ticks with honest vanilla physics in Rust** (gravity,
  drag, collisions via BlockSource, merging through a spatial hash grid) —
  no skipped ticks, same constants;
- periodic verification (sampling) of the mirror against the Java state +
  a wake channel (block events, players, inventories) — desync is
  impossible by construction;
- for players the output is 100% vanilla: the tracker sends positions from
  vanilla fields (the mirror writes back into entity fields via JNI).

## Principles (owner requirements)

- Freezing is **one mechanism among many**, only for provably static
  entities. The main win comes from algorithms / data structures /
  batching / parallelism.
- Target: **≤ 0.5 ms** for the tick of all entities at 100k+ entities and
  100k+ block entities, 20 TPS, no RAM/CPU growth.
- Compatibility: **100% vanilla behavior** for players; kill-switch fallback.

## Work order (after the baseline)

1. Baseline data (this pipeline) → ranking of real costs.
2. Rust core design driven by confirmed costs (not guesses).
3. `centities-engine` (pure core, criterion benches at 100k/500k/1M).
4. `centities-module` (weave patches per the table above + telemetry).
5. A/B on this same pipeline: Paper vs Paper+module, report in CI.
