# Parity diagnostics — session findings (2026-10-28/29)

## Diagnostic arsenal (all committed, streaming into the CI job log)

| Tool | Input | Output |
|---|---|---|
| `PH t=<tick> h=<hex> n=<n> adds=<a>` | always on in parity | first diverging TICK per scenario |
| `PD t=<tick> id:m ...` | workflow input `ph_dump` (ranges, e.g. `155-165,175-190`) | per-entity state avalanche → the diff names the CULPRIT ENTITY |
| `PTRACE t=.. x=.. vx=.. tc=.. at=..` | workflow input `trace_id` | per-tick EXACT fields of ONE entity in both legs |
| `SR t=<tick> s=<hex>` / `LR t=<tick> a=<hex>` | always on in parity (fixed in da6dd04) | per-tick state of SHARED_RANDOM / Level.random — a diff names the FIRST TICK a stream desynced (=> different DRAW COUNT that tick) |

Analyzers: `scripts/ph_diff.py`, `scripts/ph_pair_diff.py`, `scripts/ptrace_diff.py`, `scripts/sr_diff.py` (in the local workspace, not the repo).

## Fixed this session

1. **Item move-gate off-by-one** (26b86af): `ServerLevel.tickNonPassenger`
   (26.3, javap) calls `commonTick()` (tickCount++) BEFORE `entity.tick()`
   — the `(tickCount + id) % 4` gate inside ItemEntity.tick reads the
   POST value, which equals the ingest value (collect +1). The
   `.wrapping_sub(1)` fired the gate one tick late for every grounded
   idle item (run 40754993404 item 29: vanilla moved at field-tc 15/19,
   module at 16/20). **items/mixed/spread divergences moved 155→159.**
2. **SR/LR probes** (da6dd04): Level.random in 26.3 is ALSO a
   ThreadUnsafeRandom (Paper patch) — the Xoroshiro bind silently failed
   forever; `value` is inherited (walk the hierarchy); SR emitted BEFORE
   the LR bind (an LR failure zeroed both streams silently).
3. **Shipped config = vanilla AI** (3d298d1): the L22a AI engine was
   forced on by the harness (CE_MOB_AI default 1) although the plugin
   ships with `centities.mobAi` default **0** and the engine is a
   measured net loss (e774d00: mobs-25k 193.2ms with vs 129.5 without).

## Open roots (in first-divergence order on the shipped config)

### R-A: L15 AI floor desyncs both RNG streams at t=143-145
The GoalSelector microkernel still engages at mobAi=0 (`gs=` counter
>0 in every shipped leg). Bisect 40773723794 (ai_floor=0, mobAi=1):
SR/LR clean until t=232 → the floor IS the t=143-145 source. Its
staged-request protocol consumes level.random with a different COUNT
than vanilla GoalSelector (an LCG endpoint only diverges on draw-count,
not order). **Next: ship aiFloor=0 (vanilla GoalSelector) like mobAi,
keep the floor opt-in until the doc-30 pre-resolved-predicate redesign
makes the staging provably draw-exact.**

### R-B: itemcore merge-scan candidate order (items t=159, mixed t=152)
SR/LR are IDENTICAL through t>1000 for items — the itemcore divergence
is pure deterministic math, NOT RNG. Run 40780021246 PD t=159: OFF
removes id=971 and keeps fresh id=1873; ON keeps 971 (its count grew —
it absorbed something) and loses 1873.

Bytecode-verified vanilla semantics (26.3):
- ActivationRange.checkIfActive: an ItemEntity is ACTIVE only when
  `(tickCount + id) % 4 == 0` (ifne 30 → the common path; ==0 returns
  true) — the activation gate COINCIDES with the move gate. The
  module's itemCheckActive gate `(field+1+id) % 4 == 0` is EQUIVALENT
  (the guard runs pre-commonTick, field+1 == the vanilla read) — NOT
  the bug.
- ItemEntity.tryToMerge(other): if `other.count < this.count` → this
  absorbs other; ELSE (tie included) → **other absorbs this — at equal
  counts the SCAN CANDIDATE survives and the TICKING item dies**.
  All parity items are STONE x1 → the first candidate always survives.

So the sole remaining variable is WHICH candidate is first in the
scan — and it is NOT the scan order itself (fastQuery ends in
ordSortVanilla). It is WHEN the scan runs relative to the other
items' ticks:

**Vanilla interleaves per item, in EntityTickList order**:
for each item: [move tick] then [if active on its (tc+id)%4 tick:
merge scan against the world AS OF ITS TURN]. A merge applied early in
the tick removes an item that later tickers would otherwise scan, and
grows a survivor that later candidates compare counts against.

**The module batches**: the native core computes every item's move
first and itemTickFlush applies the outcomes (merge POST — commit
ed05fe4) AFTER the whole entity loop. Within-tick merge chains
therefore pair against a FUTURE world → different survivors
(exactly the t=159 971/1873 flip).

**The fix (flush redesign, next session)**: itemTickFlush must walk
the items in EntityTickList (insertion) order and, per item, apply its
own move writeback AND run its merge scan at that point (the batch
carries the computed move outcomes; the merge scans stay in Java via
MH_TRY_MERGE). One JNI crossing is preserved — only the flush loop's
internal order changes from [all moves; all merges] to
[move+merge; move+merge; ...].

### R-C: late non-RNG mobs divergence (t=183-320, AI knobs off)
With mob_ai=0 (40773720077): mobs PH diverges t=183 while SR/LR are
still clean (t=232) — a second, non-RNG bug. All-native-off (R1b):
t=320, exactly TWO entities (249, 1722) diverge in-place one tick —
candidates: L13 ticklist skip, tracker, or the ncolMh-broken push path.

### R-D: 26.3 migration — cramming audit bind broken (perf, not parity)
`lastErr=ncolMh: NoSuchMethodException LivingEntity.getRandom()` in
EVERY leg: 26.3 moved getRandom off LivingEntity → the push kernel's
cramming-audit MethodHandles fail to bind → the kernel declines →
vanilla push runs (correct but slow). Vanilla 26.3 cramming (javap):
`this.random.nextInt(4) == 0` gated on
`list.size() > maxEntityCramming - 1` — the audit uses the ENTITY's
random (SHARED), not level.random. Re-bind from net.minecraft.world.entity.Entity.

## Verified-clean subsystems
- itemcore vs RNG: items SR/LR identical through t>1000.
- push/collide knobs do NOT affect the t=143-145 stream desync.
- PH/PD integrity: re-summing the PD rows reproduces the PH hash (the
  analyzer must seed with phMix(tick) — the checker in ph_pair_diff.py
  omits the seed; cosmetic).
- vanilla-key order (ordSortVanilla) covers rowQuery; merge-scan path
  unverified (R-B).


## Session addendum (the closing hours)

### FIXED (the merge parity is GREEN)
- itemCheckActive phase per-caller (4242a2d): the dispatch sees the
  POST-commonTick tickCount (the +1 double-shifted the active gate one
  tick — the MS scan lists proved ON's item 971 scanned at ~158 while
  vanilla died scanning at 159); the rest path is PRE (+1 correct).
- After the fix: PH identical across ALL emitted ticks through t=1064
  (run 40795900185 — every tick through 400, 1-in-8 after) for all
  four scenario groups.

### THE REMAINING GAP: PH is blind to hp/fire/alive
The snapshot gate still fails on hp fields (hp: 0.0 — death diffs) at
T382+ while positions/velocities are bit-identical at every tick. The
push kernel came ALIVE this session (the ncolMh 26.3 bind fix) — its
cramming audit now applies hurtServer damage; a cramming-hurt
divergence (extra/missing nextInt(4) draw or a different hurt) is
PH-INVISIBLE because the PH hash covers only id/x/y/z/vx/vy/vz.

**Next session, step 1**: extend the PH per-entity avalanche with
hp (float bits), fire (int), alive (bool) — then the first divergent
tick of the cramming damage will name itself, and PTRACE/PD will aim
the fix. Step 2: audit pushcore's cramming branch against
LivingEntity.pushEntities (26.3 javap): `list.size() > maxEntityCramming-1
&& this.random.nextInt(4) == 0` → count non-passengers → hurtServer(cramming, 6.0f).

## Session addendum 2 (2026-10-29): ROOT FOUND AND FIXED — 5194968

### The audit, step by step (26.3 decompile, not javap guesswork)
`Level.getPushableEntities(pusher, bb)` = `getEntities(pusher, bb,
EntitySelector.pushableBy(pusher))` where the predicate folds
NO_SPECTATORS + `input.isPushable()` + BOTH-WAYS `canCollideWithBukkit`
+ team rules (26.3 Paper: the team slice rides canCollideWithBukkit,
and `LivingEntity.isPushable()` = `isCollidable(fixClimbing...)` =
**isAlwaysTicking || isPositionEntityTicking(blockPosition)** + isAlive
+ !spectator + collides + !onClimbable). The module's pushFlags mirror
already folds the same slice (probeBox-fed, javap-verified) — the
INPUTS matched only as long as they were FRESH.

### The corpse-blindness
`LivingEntity.hurtServer` deducts hp through `actuallyHurt` BEFORE
calling `this.die(source)` — from that instant `isAlive()` is false and
vanilla's per-pusher `getPushableEntities` excludes the corpse. But the
corpse moves NOWHERE for its whole ~20-tick deathTime animation:
zero box events, zero delta events → the mirror's PF_PUSHABLE stays 1
for up to 20 ticks per death. Dense pens die constantly → a standing
population of kernel-visible corpses.

### Why the RNG stayed clean while hp diverged (the decisive reasoning)
- `hits_count` gate: in a dense pen the size-gate (`> cramming-1`) is
  ALREADY open without the corpses → the extra corpses do not flip it →
  `nextInt(4)` draw counts MATCH → SR/LR identical (observed).
- `nonPass` count: vanilla counts over the FRESH list (no corpses);
  the kernel counted over hits INCLUDING them → ON's nonPass overcounts
  → `roll==0 && nonPass > cramming-1` fires hurtServer in the ON leg
  where vanilla declines → hp diverges.
- No position impact: a cramming hurt applies NO knockback velocity —
  decompile: cramming has no source position and no direct entity →
  `dealDefaultKnockback` passes xd=zd=0 → no `Math.random` draws, no
  delta → PH-pos blind, exactly the observed signature.

### The fix (5194968)
1. **`probeDie`** (CeHooks): woven as a prefix probe on
   `LivingEntity.die(DamageSource)V` (descriptor javap-verified; the
   method has no tableswitch — the patch util accepts it). At the die()
   entry hp is ALREADY ≤ 0, so the fresh `pushFlags` recompute clears
   PF_PUSHABLE mid-tick; the upsert rides the pending batch and the
   next nativePush of the SAME tick drains it (flushIfPending /
   drain_direct) — the within-tick pusher order sees the corpse exactly
   when vanilla's fresh per-pusher list does. `n_flush`'s non-pushable
   branch then drops it from the push view. Woven in
   precompute_livingentity as an INDEPENDENT optional chain (applied to
   whatever survived the guards; refusal keeps them, the failure mode
   is the pre-fix staleness — a missed refresh can only LEAVE a corpse
   in view, never spuriously remove one).
   Known inert edge: a cancelled EntityDeathEvent revives the mob with
   its mirror flags cleared until its next box probe (plugin territory;
   the parity worlds run no plugins — doc-23 deviation window).
2. **PH vitals** (BaselinePlugin): the per-entity avalanche now mixes
   fire (int), alive (Bukkit isValid), hp (DOUBLE bits — getHealth()
   is a double in the 26.3 API, the same read the snapshot prints as
   Double.toHexString). A hurt-pattern divergence now NAMES its tick in
   the PH stream instead of hiding until T382. PTRACE rows carry
   fire/alive/hp. Line formats unchanged (analyzers compatible); all
   previous PH hashes are re-baselined.

### Environment note (local toolchain rebuilt)
The workspace was reset (no cargo/JDK/Maven between sessions).
Rebuilt: rustup stable (minimal), Temurin JDK 25.0.5 (javac — the 26.3
mapped jar is class-file 69), Maven 3.9.9 partially (archive.apache
timed out; used direct javac against paper-api 26.3.build.218-stable +
adventure 5.2.0 + jetbrains-annotations + local guava instead). The
direct-javac check caught two REAL compile errors in the first PH
vitals draft (float-vs-double getHealth) before push.
