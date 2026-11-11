# doc 56 — N2-M0: the wholesale mob wire (the Java copy layer exists)

Branch `main` @ post-wave15c. Doc 42's verdict discipline said it plainly:
**no production claim until the N2 bridge runs under the existing gates**.
Waves 1–15 built and squeezed the executor (8.41 → 1.16–1.27 ms/tick p50
@ 25k on this 2-vCPU box) — but the whole engine lived only in the bench
binary: `centities-native` had ZERO JNI exports, and the only production
mob wire was the L22a per-mob floor (JNI int-array rounds per mob — the
shape the bench killed precisely because per-mob JNI is a net loss). This
wave lands **M0 of the N2 bridge**: one JNI crossing per level tick, the
mob state fully native, Java reduced to a copy layer.

## The measurement that set the direction (fresh post-wave15c numbers)

Same-window A/B, 25k × 400, 2 vCPU:

- PAR **1.27 / 1.18 / 1.16** vs SER **1.76 / 1.75 / 1.75** → −28…−34%
- vs the vanilla-8.41 bench origin: **−85…−86.2%** on 2 vCPU
- 100k × 400: PAR 4.80 vs SER 6.91 → −30.5%
- T1 2.19 / T2 1.26 → F ≈ 0.33, W ≈ 1.86 (2-point fit; the probe says the
  serial fragments are ~0.06 — the fit is conservative)
- **par(4) ≈ 0.53–0.80 ms = −90.5…−93.7%** ← the −90% goal line, four real
  cores; par(8) ≈ 0.42–0.56 = −93…−95%

The W decomposition (the probe knobs, PAR wall terms, sums EXACTLY):

| slice | ms | share |
|---|---|---|
| clone + protocol + players (the floor) | 0.25 | 20% |
| the AI floor (13 waves squeezed) | 0.56 | 44% |
| travel (bit-locked: fdlibm atan2 + IEEE sqrt) | 0.45 | 36% |

The SoA/cold-share hypothesis DIED on this box: L3 = 504 MiB — the whole
double-buffer working set (2 × 6.2 MB) is L3-resident; pass1 is compute-
bound, not DRAM-bound. The remaining slices are bit-locked or at memcpy
speed. **The bench shape is done; the gap is production-shaped.** Hence M0.

## The wire (what landed)

### Rust — the protocol (JNI-free, tested)

`centities-native/src/batch.rs` — `MobBatch`, a thin persistent wrapper
over the proven `NativeTick` (the parallel executor included):

- `new(target_seed)` / `add(eid, seed, x, y, z)` (kind reserved — M0
  builds the zombie table, `MobAI::new` does it) / `remove(eid)`
- `set_players(&[f64], &[i32])` — the flattened per-tick snapshot
- `tick(game_time, &mut ids, &mut pos) -> Ok(n) | Err(needed)` — THE call:
  one executor tick + the read-back fill.
  `ids[2i] = eid; ids[2i+1] = nav_state << 32 | no_action;
  pos[4i..] = x, y, z, yaw`. Truncation fills NOTHING and returns
  `Err(pop)` (the host grows, the next tick reads).
  **The FUSED FILL**: the parallel arm writes the read-back INSIDE pass 1
  and pass 2 (`NativeTick::tick_wire_fused` — the same alive-rank k the
  stats regions use; the drawers' words are re-written by the repair).
  A separate post-tick fill loop re-streams the whole 6.8 MB store —
  measured **+0.39 ms @25k**; the fused fill costs **+0.06–0.07 ms**
  (5.7× cheaper, ~5% of the wall). The serial/knob shapes fall back to
  the batch's own fill loop (bit-identical semantics).
- `population()` = `mobs.len() − free.len()` (exact per MobStore)
- `last_stats()` — the executor's stats borrow (NativeTick got the
  accessor; the buffer reuse contract is unchanged)

**The gate: `batch_matches_direct_executor`** — the twin world (64 zombies,
bench pile shape, the melee-magnet player), 400 ticks: the batch wire's
stats stream asserted EQUAL to the direct executor's every tick, the
read-back asserted equal to the store fields every tick (ids, nav,
no_action, x/y/z bits, yaw f32-bits), and the whole-store FNV (the 22 live
words + every goal record — the exec twin family's dump) equal at the end.
Plus the add/remove/re-add exactness, the truncation contract (Err +
nothing-filled), and the player-snapshot echo. **3/3 green.**

### Rust — the shim (`centities-module/src/mobcore.rs`)

Six dumb natives registered on HOOKS_CLASS next to the L22a wire:
`nativeMobInit(J)V`, `nativeMobReset()V`, `nativeMobAdd(IJDDDI)I`,
`nativeMobRemove(J)I`, `nativeMobPlayers([D[I)V`, `nativeMobBatch(I[J[D)I`
(returns n filled, or −population when the host arrays are small — the
tick ALWAYS happens, the read-back is pure and may lag one tick on a
population jump). Thread-local executor + scratch buffers (server-thread
confined — the same confinement the L22a wire assumes); zero steady-state
allocation on both sides of the wire.

### Java — the copy layer (`CeHooks.java`, gated `CE_NATIVE_MOB=1`, DEFAULT OFF)

- **The arm**: `precompute_mob` accepts `NATIVE_MOB_ON_INIT` (env
  `CE_NATIVE_MOB=1`) alongside `centities.mobAi`; the native-only case
  runs the LEANER `initNativeMobHandles` probe (zombie class + yRot +
  noActionTime + SHARED_RANDOM — the per-mob engine's pack classes stay
  dead weight) and arms via `setNativeMobOn(true)` → `nativeMobInit(seed
  = SHARED_RANDOM.value at arm)`. The weave is the SAME serverAiStep
  EarlyReturnGuard the L22a floor used.
- **The sight** (`nativeMobSight`, the guard's first route, BEFORE the
  L22a gates): zombie + aware (the t=157 byte-exactness lesson — the
  unaware vanilla branch draws no randomness and is never preempted) →
  first-sight `nativeMobAdd(eid, SHARED_RANDOM.value, x, y, z)` + the
  id→Mob ref map; `return true` skips the whole Java AI body.
- **The boundary** (`nativeMobBoundary`, riding `itemTickFlush` — the
  per-level entity-loop-end weave, before the n==0 return): the player
  snapshot push (PF_ALIVE semantics) → ONE `nativeMobBatch` → the applier
  (noActionTime + yaw via Unsafe same-value skips; `setPos` on the
  finite-delta change — the parked population pays nothing) → the removal
  diff (seen-last-tick vs the store's alive set → `nativeMobRemove`).
  Any throwable disarms the wire globally (the L22a discipline).
- The shipped config changes NOTHING: with `CE_NATIVE_MOB` unset the
  gate compiles to the existing behavior.

## M0 disclosures (the honesty ledger)

- **Zombie-only** (the store's goal table IS the zombie table); other mob
  types keep vanilla AI.
- The stroll stream is the executor's own chain (seeded from
  SHARED_RANDOM at arm; per-mob seeds mirrored at first sight) — the mob
  is fully native after add, its Java random is never used again.
- **Melee damage / sounds / drops / reinforcements are the N3 event
  outbox — NOT in M0.** The mobs move, target, and fight by position;
  they do not yet deal damage.
- Vanilla travel still runs for native mobs (delta ≈ 0 after the first
  ticks — the applier's positions are the truth and overwrite the
  residual each tick). The FULL body skip (baseTick/travel) is M1's
  aiStep weave; until then the e2e mspt win is bounded by the residual
  vanilla per-mob cost.
- The id maps box through `HashMap<Integer, Mob>` (~25k Integer boxes per
  tick — TLAB-cheap young-gen; revisit at M1 with a primitive-keyed
  structure if the e2e profile says so).
- Despawn keeps working through the noActionTime write-back + the vanilla
  `checkDespawn` (outside the skipped body); removal rides the guard
  saw-set (a mob of this level not seen this tick is removed from the
  store — death, discard, chunk unload all funnel through one diff).

## The live-server proof (the sandbox smoke, Paper 26.3 + the agent)

The wire was booted against a REAL Paper 26.3 server (the CRUSSTY agent,
`CE_NATIVE_MOB=1`, a flat void world, a console-summoned persistent
zombie, no bot). The full loop is alive:

```
[c-entities] natives registered
[c-entities] precomputed + armed the WHOLESALE native mob executor (N2-M0)
[c-entities] N2 probe: guard entered, native=true ok=true aware=true cls=...zombie.Zombie
[c-entities] N2 sight: first native zombie id=1
[c-entities] N2 boundary: first read-back n=1
[c-entities] N2 poll: n=1 ads=1 rem=0 appl=1417 id0=1 p=5.308..., -59.0, 24.049... jpos=5.308..., -59.0, 24.049...
```

- **ads=1 rem=0 over ~1400 ticks** — no churn; the removal driver fires
  only on real Java-side removals.
- **jpos == the native position BIT-EXACTLY every poll** — the applier
  syncs the entity to the native truth every tick.
- **Health 20.0 through 8×8 s polls** — the zombie survives and strolls
  (the positions show the engine's stroll: walk → arrive → pause → walk).

The smoke KILLED four real bugs before they could ship:

1. **the thread-local arm/tick split**: the executor was created on the
   module-init thread; the server thread saw `None` and every add
   silently returned −1 (the mob never entered the store). Fix: the seed
   rides an atomic; the executor is created LAZILY on the first native
   call of whichever thread it lands on.
2. **the kinematics tug-of-war**: the vanilla residual physics
   (gravity on a stale delta) integrated against the applier's positions —
   the re-snapped landings accumulated fall distance and dealt ~5 dmg/s
   (the NBT showed `OnGround=0`, `Motion=-0.304`, `fall_distance=0.464`,
   Health 20→15→5→death in 3 s). Fix: the applier owns the kinematics —
   Motion→(0,0,0) (one shared immutable Vec3) + fallDistance→0 + the
   position — the vanilla residual caps at one 0.08-step wobble, no
   landing, no damage.
3. **the cross-level removal kill**: `itemTickFlush` is woven per LEVEL
   (three calls per server tick — one per dimension); a GLOBAL saw-set
   let the nether/end boundaries remove the overworld's mobs (the
   observed ads/rem churn: 1276/1275 in 2551 ticks). Fix: the saw-set is
   per-level (`m.level() == level`), other dimensions' mobs are their own
   boundary's business.
4. **the removal driver itself**: the read-back diff can NEVER see a
   Java-side death (the store keeps ticking the dead id forever — the
   observed case: the mob died Java-side at +2 s, the native store walked
   the dead object for the whole run, rem=0). Fix: the GUARD saw-set is
   the driver — a mob of this level not seen this tick is removed.

## Discovered (pre-existing, NOT the N2 wire)

- `@e[type=zombie]` (the console selector) returns EMPTY with the module
  armed while the entity is alive and ticking (vanilla Paper finds it
  instantly — control run). Untyped `@e[limit=3]` matches entities. The
  `Level.getEntities/getEntitiesOfClass` rewrites (the item index's
  class dispatch) are the suspects. The CI parity is green (the harness
  does not exercise the console selector), so the reachable vanilla paths
  behave — flagged for M2's e2e stand.
- One SIGILL (ILL_ILLOPN) in the CRUSSTY runtime's class_file_load_hook
  during a LATE retransform on the very first boot (a worldgen-late
  thread); not reproducible in three subsequent boots with and without
  the mob wire. Sandbox flake — the CI is the arbiter.

## Next (M1..M3, in order)

1. **M1 — the aiStep weave**: skip the FULL Java body for native mobs
   (the EarlyReturnGuard on `Mob.aiStep`/`LivingEntity.aiStep`), closing
   the residual travel/baseTick cost; the applier becomes the sole
   mover.
2. **M2 — the e2e stand under the gates**: the parity-less perf legs
   (bot + ViaVersion) with `CE_NATIVE_MOB=1` on the mobs-25k scenario:
   the production mspt vs the module-off vanilla, JFR + the phase ledger
   on the wire (the JNI crossing must stay ~2/level-tick).
3. **M3 — the N3 event outbox**: NativeEvent (sound, drop, target set,
   damage, despawn) drained once per tick by the host applier — the
   Horizon contract; melee damage becomes real.
4. The N4 executor-boundary transcripts (native vs swarm-sim) reusing the
   parity harness's snapshot format.

## Verdict

The executor has a production entrance. The −90% line stands where doc 55
left it: **par(4) ≈ 0.53–0.80 ms (−90.5…−93.7%)**; on this 2-vCPU box the
bench floor is 1.16–1.27 ms (−85…−86.2%) with every slice measured to its
lock (clone at memcpy speed, AI squeezed 13 waves, travel bit-locked).
M1/M2 turn the wire from landed to proven-under-load.
