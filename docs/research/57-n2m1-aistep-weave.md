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

## Verdict

The per-mob Java tick body for a native mob is now ONE static call +
one HashMap probe. The M0 bounded-e2e disclosure ("the residual vanilla
per-mob cost") is closed on the wire level. Next: **M2 — the e2e perf
stand under the gates** (the parity-less legs with `CE_NATIVE_MOB=1` on
mobs-25k: production mspt vs module-off vanilla, JFR + the phase ledger;
the JNI crossing must stay ~2/level-tick), then M3 (the N3 event outbox).
