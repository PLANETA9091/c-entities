# L4 — query dominance: kill the suspend/resync gate

Follows docs 09-11 (parity gate, paired A/B, zero-JNI rows) and 13 (L2a).
Owner mandate unchanged: outputs byte-identical, never slower than vanilla.
Owner directive for this iteration: **real Rust acceleration, no new
tick-skipping** — the rest-skip machinery (L2a) stays as shipped, everything
below makes the *same work* run faster on the native path.

## The measured problem (CI, mixed-150k-mod @ 49182d2)

```
fast=772,873  mirror=18,992,328        # 96% of Level.getEntities went vanilla
susp=198 res=198 resync=18,929,862     # 198 suspend->resync cycles
reprobe=7,641/322,568                  # fast duty cycle starved to ~1/4096
row=3,890/.../rowBig=1,464,129         # row path correctly refuses dense cells
merge=719,168/0/719,168/0/201,295      # merge: 100% fast (L3 healthy)
```

The push-query family (`LivingEntity.pushEntities` → `getPushableEntities`
→ `Level.getEntities(Entity, AABB, Predicate)` — the hooked method) is
~15.8k queries/tick in mixed-150k and ~40% of the vanilla profile
(mobs-25k vanilla: `AABB.intersects` alone = 32.6%). All of it was served
by the vanilla section scan because the parity gate refused the fast path.

## Root cause: a self-sustaining suspend storm

1. World fills: bootstrap + early re-probes record expensive fast samples
   (the flush-merged apply of a huge spawn batch lands inside a probe).
2. `FAST_EMA >> MIRR_EMA` → hysteresis routes to mirror; `dt > 2*me`
   escalates `FAST_BACKOFF` to 4 → re-probe duty drops to 1/4096.
3. `maybeSuspend` counts a 16,384-decision window (= one tick in mixed):
   `chosen < 32` → SUSPENDED, index dropped (ARENA + rest states wiped).
4. The next expensive mirror query re-arms via `resumeIndexing`: a full
   resync re-upserts ~95k entities through the JNI batch (~5-15ms/tick)
   and re-poisons the EMAs.
5. GOTO 2. Measured: 198 cycles over 1,200 ticks — the module paid the
   resync tax AND served everything on the vanilla path.

## The fix (CeHooks.java only; no weave, no Rust changes)

- **Suspend machinery deleted** (`SUSPENDED`, `maybeSuspend`,
  `resumeIndexing`, `suspendCooldown`, `RESUME_NANOS`, `GATE_WINDOW`,
  `GATE_MIN_FAST`). `INDEXING` stays as a manual kill-switch, always true.
  Both paths are always exact, so routing can never break correctness —
  correctness guards (desync check, ACTIVE=false on Throwable, mirror
  fallback) are untouched.
- **Probing never stops**: re-probe duty 1/256..1/1024 (was 1/256..1/4096).
  A denser world therefore converges onto the fast path in ~20 samples
  instead of being stuck in mirror forever.
- **Backoff escalates only on a clear 4x loss** (was 2x — fill-phase noise
  starved probing into the storm).
- **EMA snaps down instantly**: a re-probe cheaper than the remembered EMA
  sets the EMA to the sample. The old 1/8 creep kept fast losing for
  thousands of ticks after fill densified the world.
- **Per-bucket telemetry** in the hooks stats line
  (`gate=w1b0:f5us150/m18us620/bk0;...`) so CI logs show the EMA race.

## Local A/B (2-core sandbox, items-100k, 600 measure ticks)

| leg | p50 | p90 | p99 | mean |
|---|---:|---:|---:|---:|
| vanilla | 151.7 | 226.4 | 312.0 | 165.5 |
| module (L4) | **38.9** | **121.4** | **139.5** | **67.0** |

−74.4% p50. Counters: `susp=0 resync=0`, merge 100% fast, row/native paths
healthy. The previous local A/B (pre-L4) had ON *slower* than vanilla
(153.6 vs 147.5) — the gate storm dominated on 2 cores.

## CI coverage change

`mobs-25k` joins the AB-pair matrix (vanilla + module back-to-back on the
same runner): it is the scenario where the push family is 32.6% of the
vanilla profile and there was no module leg at all. Expected effect there
and in mixed-150k: the ~15.8k/tick push queries move to the exact Rust
query (~3-10µs) wherever the EMA race says it is cheaper, with no resync
storms and no all-mirror phases.

## What L4 does not fix (next levers, in profile order, items-100k-mod)

1. Tracker family ~22% (`TrackedEntity.updatePlayer`/`moonrise$tick`/
   `newTrackerTick`/`sendChanges`) — Java-structural, not honestly
   Rust-accelerable without observable behavior changes.
2. Dispatch floor ~15% (`EntityTickList.forEach` + `setOldPosAndRot` +
   guard) — same; Rust cannot beat 3ns Java field writes via JNI.
3. `NaturalSpawner.createState` 2.1% — needs native type info; low ROI.
