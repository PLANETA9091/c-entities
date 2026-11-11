# doc 57 — N2-M1: the aiStep weave (the full Java body dies for native mobs)

Branch `main` @ post-N2-M0 (089179a). Doc 56's disclosure said it plainly:
with M0 the e2e mspt win is bounded by the RESIDUAL vanilla per-mob cost —
the native mob still ran `LivingEntity.aiStep` (movement scale, lerp/sensors,
jump, travel+gravity) and `Mob.aiStep`'s own body (burnUndead, the looting
scan) every tick, with the applier overwriting the kinematics afterwards.
**M1 closes exactly that: one more EarlyReturnGuard at the head of
`Mob.aiStep`, and the whole per-mob Java body is one static call.**

## The bytecode shape (javap, the real 26.3 mapped jar)

```
LivingEntity.tick()      → super.tick() (baseTick/commonTick: tickCount++,
                           fire/air) → …prologue… → aiStep()
Mob.aiStep()
  0: invokespecial LivingEntity.aiStep   ← movement scale 0.98, lerpHead,
                                            sensors, serverAiStep() @264,
                                            jump, travel(Vec3) @632
  4: BURN_IN_DAYLIGHT → burnUndead()      ← daylight burn + its RNG draws
  18: "looting" → canPickUpLoot → getEntitiesOfClass(ItemEntity) → …
```

`checkDespawn` is NOT inside aiStep — ServerLevel calls it from the entity
tick loop BEFORE the tick (`checkDespawn` profiler slice at the
`!isRemoved && !frozen` gate). Despawn survives the skip (M0's claim,
now structurally verified).

## The weave (what landed)

Rust `precompute_mob` (native mode only — `CE_NATIVE_MOB=1` without
`centities.mobAi`): chain a SECOND guard on the same stashed Mob bytes
BEFORE the existing serverAiStep guard:

- `aiStep()V` ← check `CeHooks.nativeMobStep(Mob)Z` (op-4: no-arg instance
  method, receiver = local 0 — the tickEffects/pushEntities shape; exact
  descriptor == link safety, the Paper-26.3 entity-discard lesson).
- `serverAiStep()V` ← `mobAiFloor` (unchanged: the L22a route + fallback).

Java `nativeMobStep`:

1. **Route marker** (one-shot): `N2-M1: aiStep route live` — distinguishes
   the aiStep entry from the serverAiStep fallback's probe line.
2. In-store (`NM_BYID` hit) → **saw-set feed + skip, REGARDLESS of
   awareness**. This also fixes an M0 edge: an unaware in-store mob used to
   fall through to the zero-input vanilla residual AND lose its saw-set
   feed — the removal diff then evicted it every inactive tick (re-added on
   wake with a fresh stroll seed). Now the whole population stays resident;
   the removal driver fires only on real Java-side deaths (dead mobs never
   reach aiStep at all).
3. Not in store → `nativeMobSight` (the aware-gated first-sight
   classification, byte-exact M0 semantics — the t=157 lesson: the unaware
   branch draws no randomness and is never preempted).

The shipped config compiles to the existing behavior: with `CE_NATIVE_MOB`
unset the aiStep guard is never woven (the chain lives inside the native
branch) — CI parity/e2e are byte-path-identical.

## M1 disclosures (the honesty ledger)

- For native mobs the skip removes: travel+gravity+jump (the applier is the
  sole mover — the executor owns kinematics, doc 56's tug-of-war fix),
  **daylight burn** (burnUndead + its shared-stream draws), the **looting
  scan**, sensors, look/jump control. Environmental effects that live in
  baseTick (fire from lava, drowning air, freezing) still run — Entity.tick
  is untouched. Damage dealing/dropping remains the N3 event outbox.
- `nmSkipped` tracks vanilla aiStep entries of in-store mobs — on a player-
  less console boot it accrues at the entity's activation duty cycle
  (~1/3 observed: inactive ticks run `inactiveTick`, not aiStep — the
  native executor ticks the mob regardless, which is the intended
  perf-first semantics; the applier syncs the position either way).
- The guard check costs one HashMap<Integer,Mob> containsKey per Mob per
  tick (one Integer box) for non-native mobs — the same shape the applier
  already discloses; primitive-keyed if the M2 profile says so.

## The live proof (sandbox smoke, Paper 26.3 + CRUSSTY + ce-baseline)

```
[c-entities] N2-M1: aiStep route live, native=true ok=true cls=…npc.villager.Villager
[c-entities] N2 probe: guard entered, native=true ok=true aware=true cls=…Villager
[c-entities] N2 sight: first native zombie id=41
[c-entities] N2 poll: n=1 ads=1 rem=0 skip=35   appl=103  p=8.5,-59.0,8.5    jpos=… bit-equal
[c-entities] N2 poll: n=1 ads=1 rem=0 skip=101  appl=303  p=8.5,-59.0,8.5    jpos=… bit-equal
[c-entities] N2 poll: n=1 ads=1 rem=0 skip=168  appl=503  p=8.5,-59.0,8.5    jpos=… bit-equal
[c-entities] N2 poll: n=1 ads=2 rem=1 skip=221  appl=662  p=-3.71,-59.0,2.48 jpos=… bit-equal
[c-entities] N2 poll: n=1 ads=2 rem=1 skip=287  appl=862  p=-3.78,-59.0,2.41 jpos=… bit-equal
[c-entities] N2 poll: n=1 ads=2 rem=1 skip=354  appl=1062 p=-9.10,-59.0,7.47 jpos=… bit-equal
[c-entities] N2 poll: n=1 ads=2 rem=1 skip=421  appl=1262 p=-22.03,-59.0,-5.03 jpos=… bit-equal
```

- The **aiStep route marker** proves the guard executes (a Villager —
  a non-zombie — passed through `false` to vanilla, the zombie took the
  skip).
- `skip` accrues in lockstep with the native mob's vanilla aiStep entries —
  every one of them is the full-body early return now.
- The stroll is pure native (8.5,8.5 → −22.0,−5.0 over ~1000 ticks), the
  applier syncs `jpos == p` BIT-EXACTLY at every poll, `rem=1` shows the
  diff removing a genuinely gone mob while the resident one stays.
- Clean `stop` shutdown; module suite green (4 lib + natives_parity +
  patch_entity_repro + patch_real).

## The population-scale proof (2000-mob live A/B, same box, same plugin)

Same protocol both legs (fresh flat world, ce-baseline measure window, a
console storm of 2000 persistent zombies at the spawn area, 2 vCPU, no
bot — the activation duty cycle ~11%):

| leg | mspt p50 | mspt mean | travel (last hooks@) | gs (last hooks@) |
|---|---:|---:|---:|---:|
| A — vanilla AI | 69.0 | 62.9 | 817 318 | 806 756 |
| B — native M1 wire | 50.0 | 49.8 | 26 860 | 66 050 |

**−21% p50 / −26% mean at the WORST-case duty** — the vanilla mobs sit in
`inactiveTick` 89% of ticks (no player), while the native executor ticks
its population fully every tick and still wins the window. The movement
pipeline counters collapse 30× (travel) and 12× (GoalSelector) — the skip
is visible at population scale, not just in the polls. The M2 bot legs
(100% duty, 25k mobs) are where the full −90% executor line shows.

### The liveness removal driver (the saw-set proxy dies)

The 2000-mob smoke caught a real M0-era edge the single-mob smoke could
never see: at the activation flip the WHOLE population swapped —
`ads+2000 rem+2000` in one window. Root cause: the removal driver
diffed "mobs the guard did not see this tick", but an INACTIVE mob never
runs aiStep, never feeds the saw set — the diff then evicted it; the
re-add on its next active tick reset every stroll seed.

Fix: the diff now reads the Java object state directly at the boundary —
`sm.isRemoved() || !sm.isAlive()` (isRemoved covers death/discard/
chunk-unload; isAlive covers the death animation via deathTime/health).
Activation-independent, and it catches a Java-side death on the SAME
boundary tick. Post-fix soak: `ads=2008 rem=0` for the whole run (8
natural spawns joined, zero churn), positions bit-exact, 0 boundary
failures.

## Verdict

The per-mob Java tick body for a native mob is now ONE static call +
one HashMap probe. The M0 bounded-e2e disclosure ("the residual vanilla
per-mob cost") is closed on the wire level. Next: **M2 — the e2e perf
stand under the gates** (the parity-less legs with `CE_NATIVE_MOB=1` on
mobs-25k: production mspt vs module-off vanilla, JFR + the phase ledger;
the JNI crossing must stay ~2/level-tick), then M3 (the N3 event outbox).

## Addendum — M1.6 (the wire fastpath) + M1.5 (the tick weave)

The 2000-mob JFR profile (server-thread self-time) drove two more waves:

**M1.6 — the wire fastpath.** `nativeMobStep` self 6.4% → 0.0%,
`nativeMobBoundary` self 6.7% → 1.9%:
- `NM_BYID` became `NmMap` — a primitive int→Mob open-addressing map
  (linear probing, Knuth 6.4 R cluster-pull deletion, grow at 50%): the
  old `HashMap<Integer,Mob>` boxed twice per mob per tick.
- The saw-set machinery died outright (the liveness driver made it dead
  code): no `NM_SAW` map, no per-mob `HashSet.add`, no `NM_SEEN`/
  `NM_SEEN_TMP` swap, no boxed adds per boundary.
- The liveness check folded INTO the apply loop: a dead mob is collected
  into an int scratch and drained after — the steady state (nobody died)
  costs one extra field check per mob and ZERO map walks (the two full
  `NM_BYID` walks per boundary are gone).

**M1.5 — the tick weave.** A THIRD guard chained at the head of `Mob.tick`
(op-4, `nativeMobTick`): the native mob's whole Java tick dies —
`Entity.tick` (baseTick: fire/air/lava/tickCount, pushEntities) and every
layer above (the M1-skipped aiStep, updateControlFlags, the swing/item-use
machinery). The classification moved here (aiStep is skipped for natives —
the sight could never fire there); the applier keeps `tickCount`/
`ticksLived` advancing with one Unsafe increment per mob per boundary
(one boundary per server tick — pinned empirically: appl grows exactly n
per tick). The death path is re-verified: the sight refuses `!isAlive()`
mobs (with the classification at tick ENTRY a dying mob would otherwise be
re-classified and resurrected — a corpse the vanilla dead-branch could
never remove); after the liveness diff drops a dead native, the next
tick's guard declines and LivingEntity.tick's dead branch animates and
removes the corpse. Live: `/kill` → `rem=1` on the same boundary, the
survivors untouched (jpos bit-exact, zero churn).

**M1.5 disclosures**: native mobs no longer burn in daylight, take lava/
fire damage, drown, or freeze (baseTick skipped) — environmental death
returns with the N3 event outbox; `updateControlFlags` freezes (the
vanilla goalSelector is not used by natives); hurt-by-command still kills
(hurt runs outside the skipped body, health drops, the liveness diff
fires). JFR after both waves: server-thread samples 999 → 790 → 504 over
equal windows; the remaining mob-attributable server time is the applier
(boundary 13.5%) and the tracker's data reads (~8%, needed for real
players).

## Addendum 2 — the first production A/B (the real runner, the real bot)

`scripts/run-baseline.sh` executed locally end-to-end (2 vCPU sandbox, 4 GB
box, the mineflayer bot via ViaVersion — a PLAYER in the world, 100%
activation duty, the plugin fill: 10000 persistent helmeted zombies around
the spawn, measure window 300 ticks, JFR):

| leg | MSPT p50 | p90 | mean | TPS est. |
|---|---:|---:|---:|---:|
| vanilla (module off) | 174.7 | 313.9 | 179.9 | 5.7 |
| **native M1.5 wire (CE_NATIVE_MOB=1)** | **49.6** | 119.0 | 65.1 | **20.0** |

**−71.6% mspt p50 at 10k mobs, player-present, full duty.** The wire state
across the whole native run: `n=10000 ads=10000 rem=0` — every fill zombie
classified at its FIRST tick (the M1.5 route), zero churn, zero boundary
failures, jpos bit-exact. The gap to the −90% line at 10k (17.5 ms) is the
tracker's data reads (~8% of the server thread at 2k — scales) plus the
applier's setPos for a walking population — the M2 CI legs (4 vCPU, 25k,
mobs-25k-native matrix row added) carry the next measurement; the
tracker/boundary shave is the next wave if the 25k numbers demand it.

Infra notes: background processes are reaped between tool calls in this
sandbox (the whole leg must run inside one call); a double `PATH=` in one
`export` statement expands both against the ORIGINAL PATH — the runner
silently got Java 21 and Paper's gate refused it; separate export lines.
ViaVersion/ViaBackwards resolved from Hangar (the GitHub release asset
names 404 on `latest/download`).
