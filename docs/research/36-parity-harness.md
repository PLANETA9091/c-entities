# 36. The parity harness — the honest behavioral A/B

Date: 2026-10-27. Status: SHIPPED (9b76ee4 + c16dcc3 fixes). Purpose:
ab-paired proves SPEED; parity proves BEHAVIOR — "the mobs and everything
do exactly what Paper's mobs do".

## Design

Each scenario runs TWICE on the same runner — vanilla Paper (MODULE=off)
and Paper + c-entities (MODULE=on) — under a deterministic-replay regime:

- fixed `level-seed` in server.properties (byte-identical terrain);
- NO bench bot (player motion is wallclock-random — it would desync the
  legs);
- the plugin's spawn layout is seeded (`Random(42L)`) and the fill window
  starts at a FIXED tick (T100), not at bot-join;
- `-XX:+UnlockExperimentalVMOptions -XX:hashCode=5` on both legs (the
  identity-hash determinism belt; the flag is experimental on JDK 25 —
  the unlock flag must PRECEDE it or the JVM refuses to start).

### The one-shot SHARED_RANDOM seed (the determinism lever)

javap (26.3): all entities share ONE RandomSource — `Entity.SHARED_RANDOM`,
an `Entity$RandomRandomSource` (package-private final class extending
moonrise's ThreadUnsafeRandom). Its `setSeed` is `public synchronized` but
the DECLARING class is not accessible from a plugin without
`setAccessible(true)`; more importantly the source **LOCKS on its first
setSeed** — every later call is logged ("Ignoring setSeed on
Entity.SHARED_RANDOM") and IGNORED.

So the harness seeds it EXACTLY ONCE, in onEnable (before any entity
exists), and never again. That one seed is the whole lever, and it is
sufficient: the fixed-tick fill window pins the spawn schedule to
identical tick numbers in both legs, spawn construction draws and
per-tick AI draws then evolve the single global stream identically
tick-by-tick. Any behavioral difference anywhere (an extra or missing RNG
draw, a different goal decision, a movement drift) diverges the stream
and never re-converges.

### Snapshots

Full observable entity state written with EXACT bit patterns (hex
floats/doubles — no decimal rounding), sorted by entity id: id, type,
x/y/z, yaw/pitch, velocity, onGround, fire ticks, alive, health,
noActionTicks, item amount + pickup delay. Taken at fill-done+50,
fill-done+300 and at finish. Each snapshot logs `PARITY-SNAP tag=... n=...
sha256=...` — the CI log greps can compare hashes directly.

### The gate

`scripts/parity-compare.py` diffs the snapshot sets: entity count,
per-type counts, id sets, then bitwise field equality (tol=0; `--tol D`
exists for FP-order debugging). Exit 1 on ANY divergence. Four scenarios
(mobs 2500 / items 20000 / mixed 15000 / spread 10000) x (off, on) legs.

## The first run's three failures (all harness-side, fixed at c16dcc3)

1. `IllegalAccessException` on setSeed (package-private declaring class)
   — both legs ran on their own entropy streams. Fix: setAccessible(true).
2. Per-fill-tick reseeding was impossible by design (the one-shot lock) —
   removed; the single onEnable seed replaced it.
3. The fixed-tick start task fired EVERY tick: parity mode never set
   `started=true` (only onJoin did), so begin() re-ran per tick — chunk
   tickets re-added, a NEW fillStep timer per tick, the phase ping-ponged
   FILL<->WARMUP and fillDoneTick receded ahead of its own snapshot
   offsets forever; the leg hung to RUN_TIMEOUT with no baseline.json.
   Fix: one-shot (started=true before begin()) + the 45s failsafe disabled
   in parity mode (a wallclock begin starts the fill at a different tick
   per leg).

## What the gate catches

Anything that changes observable behavior: wrong goal selection, extra
RNG draws, 1-ULP movement differences from reordered float math, missed
entity removals, item merge drift. It does NOT catch invisible changes
(private counters, internal caches) — by design: those are not behavior.
