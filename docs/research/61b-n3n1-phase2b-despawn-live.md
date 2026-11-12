# 61b — N3-N1 phase-2b: the despawn decision is LIVE (the executor slot, the mirror roll, the cause-column outbox)

Date: 2026-11-12. Status: IMPLEMENTED (Rust twins green, natives_parity
green, the whole workspace suite green; the CI player-present scenario is
the remaining honest gate, see §"the validation gap" below).

## What shipped

The doc 61 §"the native pieces" list, phase-2b scope:

1. **The executor slot** — the decision rides the per-mob eval
   (`re_eval_mob`), BEFORE the AI (the vanilla `lambda$tick$0` order:
   checkDespawn -> guardEntityTick(tick); javap-pinned from the
   paper-26.3-249 mapped jar). A despawned mob skips its tick entirely
   (the ledger: the final-tick AI writes land on a removed entity —
   vanilla still ticks the mob once after the discard; the difference is
   unobservable state on a discarded entity, covered by the N3 event
   ledger below). The serial walk and the wave15 parallel pass 1 run the
   IDENTICAL body — the decision is per-mob pure (cfg, difficulty,
   players snapshot, the mob's own state), so bit-identity between the
   arms holds by construction (the strata twin drives both arms).
2. **The RNG roll** — `random.nextInt(800)` draws from the mob's OWN LCG
   mirror chain INSIDE `DespawnCfg::decide` (bc 270-282: the roll runs
   IFF soft-hit && !persistent && noActionTime > 600, BEFORE the
   removeWhenFarAway check). The doc 61 §4 walk-order note was written
   for the shared-stream frame; under the shipped per-mob mirror frame
   (the batch.rs honesty note — the mob's Java random is never used after
   first sight) the order requirement is trivially satisfied (a per-mob
   chain has no cross-mob order) and — the important part — the wave15
   parallel executor stays UNTOUCHED: the despawn roll is per-mob state,
   never a shared-stream drawer, no prefix assignment, no pass-2 repair
   class. bound=800 is even, so the Java rejection loop never fires —
   exactly ONE 48-bit LCG step per roll (the draw-order twin pins the
   divergence against the decision-OFF arm).
3. **The outbox** — the doc 61 §5 `nativeMobDiscard(eid, cause)` became
   the CAUSE COLUMN on the read-back rows: ids[2i+1] bits 48..56 carry
   1=Peaceful / 2=Hard / 3=Soft for the tick the decision fired (the row
   itself carries the pre-tick state). ZERO extra JNI crossings, the
   applier applies `mob.discard(EntityRemoveEvent.Cause.DESPAWN)` on the
   tick thread and feeds the existing removal driver (nmCollectDead ->
   nativeMobRemove + NM_BYID.remove) in the same drain. The packing:
   no_action stays the LOW 32 bits (the applier's contract unchanged),
   nav_state moved to bits 32..48 (the M0 wire never read it), the cause
   rides 48..56. The slots tombstone in `finalize_despawns` AFTER the
   fill (the fused parallel path finalizes inside tick_wire_fused; the
   serial fallback finalizes after its own fill — which also fixed a
   LATENT DOUBLE TICK: the old serial fallback re-ran exec.tick after
   tick_wire_fused had already run the walk).
4. **The predicate bits** — the boundary computes the exact
   PLAYER_AFFECTS_SPAWNING predicate per player per tick (javap
   EntitySelector lambda$static$7: `!isSpectator() && isAlive() &&
   affectsSpawning` — the affectsSpawning field is PUBLIC) as NM_PF bit
   1. The executor's nearest scan reproduces getNearestPlayer exactly:
   strict-< on the 3D squared distance, the FIRST player in the snapshot
   order wins ties (bc 102-105: `d15 >= d10 -> skip`), the snapshot is
   filled in level.players() order.
5. **The virtuals** — resolved ONCE at classification (doc 61 §3's
   class-cache design, simplified to per-mob): persistenceRequired ||
   isPassenger() || isLeashed() (requiresCustomPersistence, javap bc
   0-19), removeWhenFarAway(0.0) (the base Mob body returns TRUE, javap
   bc 0-1; zombies never override it), and the TriState
   despawnInPeacefulOverride.toBooleanOrElse(!type.isAllowedInPeaceful())
   (the field and the resolution are exact — a PUBLIC field). The bits
   ride the nativeMobAdd `kind` param (bits 8/9/10 + the MobCategory
   ordinal in bits 0..7). Ledger line: a mid-life leash/persistence flip
   is invisible until re-classification — the M2c class-cache precedent.
6. **The difficulty** — the PEACEFUL gate (bc 0-45) reads the Difficulty
   ordinal that rides the batch call PER TICK (`nativeMobBatch(int
   gameTick, int difficulty, ...)`): a runtime /difficulty change is
   exact on the next boundary, no re-push machinery.

## The config table

The phase-2a push (`pushDespawnCfgOnce`, once per level, WeakHashMap-
guarded) now lands the table on the executor too (mobcore's
n_despawn_cfg -> with_exec set_despawn_cfg). The decision stays OFF
until the push succeeds (DESPAWN_CFG_LIVE=false -> the M2c status quo —
the SAFE default). Multi-level: the LAST pushed cfg rides the single
executor — the M0 one-executor ledger (doc 56 §3); the bench/parity
worlds are single-level and exact.

## The validation gap (updated)

Doc 61's honest blocker stands: no existing scenario can SEE a native
despawn (parity has no player; the bench is persistenceRequired). The
Rust side now carries the strata twin (the batch wire vs the direct
executor over a world designed to fire EVERY branch: hard, soft-with-roll,
missed-roll, persistent, no-affects-player, in-soft-reset — the event
rows asserted on the read-back, the whole-store dumps asserted equal
after the tombstones) + the draw-order twin (the roll lands before the
AI draws, pinned against the decision-OFF arm) + the no-player gate
twin. What CI still needs: the player-present, NON-persistent bench
scenario with strata inside/outside the ranges (the doc 61 §"the
validation gap" spec) — the next phase-2c item.

## Cost

The decision is O(players) per mob per tick (the nearest scan + the gate
compares), all inside the existing pass — the bench (25k, the decision
OFF) reads p50 1.48 ms/tick unchanged; the live wire pays ~one f64
distance compare per player per mob + one branch. The Java side pays one
JNI crossing per despawn EVENT (rare by definition) — nothing per tick.

## Phase-2d addendum: the per-level executor split

The wire's itemTickFlush is woven per LEVEL — a default-properties boot
(nether + end alive) fires the boundary 3x per server tick, and the ONE
executor ticked the WHOLE store 3x (the mob AI advanced 3x per server
tick on any real server; the walk counter hid it — the saw-set is
per-level and the other dimensions' tick lists are empty — and the jpos
poll is self-consistent by construction). The split:

- mobcore's EXEC becomes `HashMap<level_id, MobBatch>` — each Level
  ticks its OWN executor; the players snapshot, the despawn cfg
  (paper-world.yml IS per-world state) and the Difficulty are per-level
  too. The executor is created lazily on the level's first crossing.
- The small level id: `nmLevelKey(Level)` — a WeakHashMap-assigned
  stable int per Level object, riding EVERY wire crossing
  (nativeMobAdd/Remove/Players/Batch/DespawnCfg all gained the leading
  `int level`; the natives_parity guard pins the descriptors).
- nativeMobInit stays global (the stroll-stream seed at arm); each
  level's executor clones it; nativeMobReset drops ALL levels.

The bench/parity worlds stay single-level and exact; the 3-dimension
boots are now correct BY CONSTRUCTION (the 3x ticking hole is closed,
not ledgered).
