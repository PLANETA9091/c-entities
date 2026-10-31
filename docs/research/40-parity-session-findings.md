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

## Session addendum 3 (2026-10-29): the push-trigger harness trap (98483d2)

The die-probe parity run (40881390981, head 5194968) STILL failed — and
the failure was NOT the die-probe: the OFF leg's snapshot shas were
byte-identical to the previous run's (dee2a1edbd at T382), so vanilla
was vanilla; the ON leg had n=2507 vs 2509 and a NEW error line:
`lastErr=mobAiFloor: NPE ... VH_GS_DISABLED is null` (36 occurrences;
the previous run had ZERO and mob=0/0/0/0).

### The chain
1. `parity.yml` resolved the mob_ai knob as
   `[ inputs.mob_ai = "0" ] && 0 || 1` — on a PUSH trigger the input is
   EMPTY → MA=1 → `-Dcentities.mobAi=1`: **every push-run armed the
   experimental L22a engine that ships OFF.** The previous session's
   green-PH runs were DISPATCHED with mob_ai=0 explicitly — the trap
   never fired for them.
2. The engine's `initMobAiHandles` resolved the Mob-side handles but
   NOT the GoalSelector RefFields (goalTypes/lockedFlags lived only in
   the L15 `initAIHandles`, which never runs when aiFloor ships off).
   The first zombie classification therefore threw NPE MID-TICK —
   after `noActionTime++` and `sensing.tick()` (stepped=true) — the
   mirror-tail recovery ran on a half-mutated mob, vanilla
   serverAiStep was skipped for that tick, and the leg desynced.
3. Timeline check: the NPE at t≈158 (first hooks@ with lastErr), the
   PH divergence at t=161 with n/adds still EQUAL (one mob's state
   drifted), n permanently short by 2 at T382.

### Fixes (98483d2)
- `parity.yml`: EMPTY now resolves to the INPUT'S DEFAULT — mob_ai
  arms only on an explicit `1`. item_tick/merge keep empty=1.
- `initMobAiHandles` resolves goalTypes/lockedFlags itself: the engine
  is self-contained with aiFloor=0 (a legitimate dispatch combination).
- `mobAiFloor` fail-fasts on `VH_GS_DISABLED == null` BEFORE any
  mutation — a mis-configured engine declines, vanilla runs, exact.

### Analyzer note
`scripts/ph_diff.py` mis-split the legs on this log (it keyed on a
stale "parity leg" marker format) and reported IDENTICAL across 140
ticks while the PH lines at t=161+ differed. The inline segmenter
(new leg starts when the tick counter resets) is the reliable split;
ph_diff.py needs the same rule.

### L41 rides in 98483d2
The merge predicate's max-stack pre-resolution (doc 41 move 1):
`ItemInstance.getMaxStackSize()` (8.29% of items-100k) walks
DataComponents per pair; when `PatchedDataComponentMap.patch` is the
shared fastutil EMPTY singleton the walk is exactly
`prototype.get(type)` — a per-type immutable constant cached by
identity (two-slot, server-thread confined). Proof chain: the ctor
assigns `Reference2ObjectMaps.emptyMap()` (javap: getstatic+areturn —
the global singleton); `ensureMapOwnership()` REPLACES the patch object
before the first in-place mutation; `copy()` shares only under
copyOnWrite (the first set detaches). Every patched stack (NBT
components, plugin set()) fails the singleton check → the exact
virtual walk. EMPTY-stack semantics preserved (count==0 → max 1 →
mergeable, guarded at the call site).

MEASURED (profile run 40886854724, mixed-150k with L41):
`ItemInstance.getMaxStackSize` dropped from the items-profile's 8.29%
to **0.85%** (#14). The new top bucket is
`AABB.intersects(DDD)` at **8.30%** — the L37d exact re-check of the
native query's MARGIN SUPERSET (`marginExtra` = 226.4M candidates per
300-tick measure window on mixed-150k!). The next speed move (doc 41
update): make the native query exact (f64 boxes in the index) or SIMD
the re-check — 226M Java-side re-checks is the whole remaining item
tax.

## Session addendum 4 (2026-10-29): the "green PH" was FALSE — the real roots

The die-probe/L41 parity runs failed, and the investigation exposed
that **the previous session's "PH identical through t=1064" was a false
green**: the ph_diff analyzer's regex never matched the ct= PH format
(df2d659 added the field), zero lines were compared, and the broken
inline fallback mis-paired the legs. The OLD head (443b8dc) and ALL new
heads diverge IDENTICALLY:

| scenario | first PH divergence | shape |
|---|---:|---|
| items | t=159 | off={1873} on={971} — n equal, changed=0 |
| mixed | t=152 | off={469} on={964} — same shape |
| mobs | t=183 | value diff, n equal (the doc's R-C) |
| spread | t=193 | value diff, n equal |

The items/mixed shape is the R-B survivor flip: the MS dump shows the
ON leg's scan of **self=971 returning n=0** where vanilla's scan of the
same item found 1873 (26.3 ties: the CANDIDATE absorbs the ticker —
971 died in vanilla; in the module 971 survived and 1873 died to the
counter-scan). **NOT an order problem — a query false negative.**

MSQ (fdfe7ab) now prints, per merge scan of MERGE_SELF at PH_DUMP
ticks: the serving path (row/native/mirror), the RAW pre-predicate
candidate ids captured in BOTH the native resolve and the row path
(before window/home/box/predicate rejections), and the
win/home/stale skip deltas. The next dump run names the stage that
drops 1873.

Analyzer additions: `scripts/pd_diff.py <log> <tickA> <tickB>` — the
per-entity PD diff across the divergence boundary (named 971/1873 and
469/964 directly); ph_diff fixed (ct= format + marker-baseline reset +
tick-reset segmentation for markerless logs).

## Session addendum 5 (2026-10-29): the pile sentinel (49caa57) + the mobs travel root

### items/mixed — ROOT CAUSE FOUND AND FIXED (the dense-kill sentinel)
The MS dump showed scan after scan returning n=0 INSIDE the pile (29
scanners at CT=475, all n=0 except one). The pile (area=0 — every item
spawns at the anchor) makes its cell cross ROW_CAP=510 before the first
publish; the dense-kill wrote its sentinel ONLY into a non-null slot;
the cell's slot stayed NULL forever; the row path's null contract
("never occupied = an EMPTY cell", freshSeen satisfied by the other
covered cells) answered an EMPTY candidate list for every scan in the
pile — a "successful" empty answer, no vanilla fallback, the merges
silently dropped. FIX 49caa57: allocate the row array for a dense cell
and write the sentinel unconditionally (Java reads count=0x7FF >
ROW_CAP -> rowBig -> the native path).

The earlier L41b row expansion (9fb3648, the 1-cell single-homing
coverage) is kept: it closes a REAL coverage gap vs vanilla's
box-overlap section registration, but it was not THIS divergence.

### mobs/spread — the roots are NAMED, the fix is the next session
The PTRACE of 377 (mobs) at t=183 — the FIRST-MOVE tick after 12 ticks
standing still (the AI's first stroll, the yaw identical in both legs —
the RNG clean):
  off: vx=0.0202 vy=-0.078 vz=0.0202   (the ground friction applied,
                                        the gravity delta kept)
  on : vx=0.046  vy=0.000  vz=0.046   (the full air speed, vy zeroed)
The L20/L28 native travel classified the starting mob as NOT-on-ground
(no slipperiness, the collision zeroed vy) while vanilla ran the ground
branch. Every subsequent tick compounds the position error (the
spread's 2182 shows the same class one scenario later). NEXT: audit
mobTravelNative's ingest/onGround/friction classification against the
26.3 LivingEntity.travel(bytecode) — the first-move tick is the
reproducible shape (a standing mob's first stroll input).

### MSQ findings (run 40904510292, the streamed MSQ lines)
- `MSQ t=475 self=971 path=row rawN=0 win=163 home=0 stale=0` — 971's
  scan: the ROW path staged rows, the packed x-window rejected ALL 163
  candidates, ZERO derefs. The merge partner 1873 was never derefed.
- `MSQ t=475 self=1384 path=row rawN=1 ids=[1873]` — the NEIGHBOR scan
  (the row path, a different query box) DID find 1873: the index row
  contains 1873, its packed window passes for 1384's interval.
- `MSQ t=475 self=1383 path=native rawN=1 ids=[1637]` + `MS n=0` —
  1383's scan went NATIVE (the row refused), the index returned ONE
  candidate (1637), and the final list was EMPTY — 1637 was dropped by
  the exact re-check or the predicate.

The asymmetry (the row finds 1873 for 1384; the native misses it for
1383; 971's row window rejects everything) points at the staged-row
WINDOW geometry — the L41e diagnostic (598f8fe) dumps the staged rows'
(cx, cz, n) per MSQ (`MSR ... rows=[(cx,cz,n),...]`), which pins the
exact staged-cell/window disagreement. The sentinel fix (49caa57) is
CONFIRMED working (the dense pile cells now refuse the row path —
path=native for the in-pile scans).

Analyzer notes: the streamed job log filters through run-baseline.sh's
grep — MSQ/MSR were invisible until added to the pattern (883c8aa).
The 0f3e16e commit shipped a non-compiling sampler (the field-name
mangles msqRej/MSQ_REJ) — fixed in 2ce9476; always compile-check the
Java before push (the local direct-javac harness catches it).

## Session addendum 6 (2026-10-29): the first-move root was the PUSH MIRROR,
### not the travel classification

The addendum-5 verdict ("the L20/L28 native travel classified the starting
mob as NOT-on-ground") was WRONG about the mechanism. The native travel
NEVER ran on the diverging tick: mobTravelNative requires input==0 (the
guard's first gate), and the first stroll input is non-zero — vanilla
travelInAir ran in the ON leg and wrote the correct
(x*0.546, -g*0.98, z*0.546) tail. The trace's vy=0.000 was UNREACHABLE
from travelInAir (its tail always subtracts gravity post-move) — the
delta was OVERWRITTEN AFTER travel by the L12 push kernel's self
writeback.

THE CHAIN (end-to-end, 26.3):
1. Entity.move() ends with setDeltaMovement(the clipped delta) — the
   collided axes zeroed: (0.037, 0.0, 0.037) on the first-move tick.
2. The travelInAir tail then writes (0.0202, -0.0784, 0.0202).
3. probeDelta was a PREFIX reading e.getDeltaMovement() — the PRE-write
   field. Every setDeltaMovement therefore batched the value the write
   was ABOUT to replace; the drain applies the events in order and the
   LAST event per entity wins.
4. The last event before the kernel is the TAIL's own prefix capture:
   the PRE-tail field = the post-move clipped delta — vy 0.0 (the
   collision zeroed it), vx un-frictioned. The box event (setPos ->
   batchUpsert) captured the same pre-tail moment.
   For a 12-tick STANDING mob the tail write (0, -g*vf, 0) is
   value-identical every tick — the L20b gate skipped it AND its probe
   event, leaving only the box-event captures: the mirror was fed
   (0, -0.0784, 0)-then-(0, 0, 0)-class values, never the tail.
5. n_push's self writeback then REPLACED the fresh vanilla tail with
   (stale base + push): vy=0.0, the un-frictioned vx — exactly the
   addendum-5 trace ("the collision zeroed vy", "no slipperiness").

SECOND HALF: the pushed CANDIDATES' forces lived only in the mirror
(kadd ledger); the candidate's own Java field got the force only when
ITS OWN kernel run writebacked — lost whenever that run declined
(the density gate, the mirror miss, the crammed route). The NEEDSSYNC
flag was set but never consumed by any Java code.

FIX (L42, this session):
- probeDelta on setDeltaMovement(Vec3) is now VALUE-CARRYING: the 26.3
  body assigns the ARGUMENT (an isFinite-gated write), so the probe
  batches the post-write field value. The DDD overload delegates to the
  Vec3 one, so its probe stays wake-only (the inner value event lands
  after it and wins the drain). Non-finite values are never batched
  (the vanilla body skips the write; the gate-off exactness guard).
- pushcore emits the pushed pairs (id, ddx, ddz) instead of mutating
  the candidate's mirror; pushDispatch applies them THIS TICK — the
  exact vanilla doPush order (entityx.numCollisions++ via the new
  O_NCOL Unsafe offset, then entityx.push(DDD) — the no-event 3-arg
  overload, needsSync included). The write's own probe event re-syncs
  the mirror before the next kernel read (every dispatch flushes
  first). The kadd ledger is dead and removed; PF_NEEDSSYNC survives
  only on self (diagnostics).
- The kernel's self base is now the LIVE field: the writeback is
  field + force, the friction/gravity tail survives.

KNOWN REMAINING MICRO-GAP (pre-existing, unchanged by L42): a
cramming-audit kill on the PUSHER inside the same dispatch — vanilla
runs the audit BEFORE the pair loop and a dead pusher's isPushable
skips the self-force, while the kernel computed the self base
pre-audit. A >23-neighbor, 1-in-4-roll event; the mobs scenarios have
not shown it. The candidates' own pushes are unaffected (the audit
never hurts them).

### L42b correction (run 40921675568): the pair arrays must be written IN PLACE

The first L42 run MOVED the mobs divergence EARLIER (t=183 -> t=170,
the pd pair 58/217) — a regression, not a fix-exposing shift: the
pushDispatch pair apply missed 210/223 ARENA resolves (pushPairMiss).
Root cause: PUSH_PAIR_IDS/PUSH_PAIR_D were `static final long[]/double[]`
and n_push replaced the FIELD VALUE with a fresh array per call
(SetStaticObjectField). The JIT constant-folds a static-final array
REFERENCE in the compiled loop, so every compiled read kept the
ORIGINAL empty arrays (the first ~13 interpreted reads landed, the
compiled ones read all-zero ids). Fix (0261d4f): GetStaticObjectField +
Set{Long,Double}ArrayRegion on the EXISTING array object; PUSH_PAIR_N
(non-final int) stays the count gate. Lesson: JNI-written results go
into preallocated non-replaced buffers when the Java reader can be
constant-folded — the same discipline as the AI_IN/AI_OUT and the
query out-buffers.

## Session addendum 7 (2026-10-30): the flush-time scan view — ROOT PROVEN (L44, d01f631)

### The instruments that closed it
1. **MS tc/mv/src** (59f134c): every merge scan now prints the cadence input
   (tc = e.tickCount == the Rust st.tick_count), the floor-change verdict
   (mv = the new B_MERGE2 output bit), and the caller (src: flush vs the
   vanilla-body guard).
2. **QB** (f4360ed): the exact query box + radius per MSQ scan (the marker
   had to be added to run-baseline.sh's stream grep — same trap as MSQ).
3. **PTRACE id 1827** (41083916822): the per-tick fields of the merge
   partner — bit-identical across both legs, which killed the counter-phase
   theory for good (the PTRACE t=150..158 of item 971: ZERO field diffs).

### The exact-number proof (items t=159, the 971/1827 pair)
- 971's scan at ct=475: QB Y=[-59.2439, -58.9939] (0.25 tall — the merge box
  Y-inflate is 0 either way), mv=1, tc=10, 13 window-passed candidates, ALL
  box-rejected, n=0.
- 1827: spawned t=159 (tc=1), vy=+0.1568 (rising). Its POST-body box
  y=[-58.84, -58.59] is 0.153 ABOVE the window — our flush-time box test
  rejected it CORRECTLY for what it could see. Its PRE-body (spawn) box
  y=-58.997 WAS INSIDE the window.
- Vanilla's 971-scan runs DURING 971's body; 1827's body runs LATER
  (id order) — vanilla saw the spawn box, merged (tie: the candidate
  absorbs the ticker → 971 died — exactly the OFF-leg PD).
- The flush runs AFTER the whole entity phase: 1827's body already moved
  it one physics step past the vanilla view. Falling/rising young items
  step 0.1-0.3 blocks/tick; the 0.25-tall Y window makes the step decisive.
- mixed t=152 (469/830): the same mechanism, opposite sign — 830's spawn
  box was OUTSIDE vanilla's 469-scan view, its post-body box INSIDE ours →
  our extra merge. spread t=210: the miss variant. mobs t=275 (the SR/LR
  divergence at 321-394 downstream, the PH divergence WITHOUT any RNG diff)
  is the push mirror's own flavour of the same flush-timing class.

### The fix (d01f631)
- `appendItem` captures the PRE-body AABB of every entry (IB_PRE, the same
  side-packed cd-slot; pass 1 keeps overwriting IB_CD with the POST state
  for the tick compute).
- The apply pass builds the VIEW: RESYNC + FORCE_VANILLA entries (their
  bodies ran in-phase), inactive excluded — id -> (PRE box, batch idx).
- A scanner at batch index i sees an entry j>i at its PRE box: the row
  path never window-skips it (its packed interval rides the POST box) and
  skips the home re-check (vanilla registers a crosser in BOTH sections);
  the native path's exact re-check substitutes the PRE box the same way.
- fastQuery refuses the native resolve when an AFTER-scanner view entry's
  PRE box touches the query (the index boxes are POST-move — the answer
  would be view-wrong). The view is apply-pass-only state (IN_ITEM_APPLY).

### Known residuals (accepted, documented)
- A row-path REFUSAL (dense cell/multi-cell) coinciding with a fresh spawn
  in range falls to the mirror, which is equally view-stale — needs a
  dense-pile scan to matter; not observed on the parity scenarios.
- The merge's age/pickupDelay side effects land one body-step later than
  vanilla's (max/min commute with the increments for young items; the
  snapshot gate does not hash age/pickupDelay).
- The mobs push-pair divergence (t=275, entities TBD by PD at 270-280) is
  the NEXT root — same flush-timing family, push mirror side.

## Session addendum 8 (2026-10-30): L44c lands — two roots down, the map for the rest

### Fixed this session (runs 41087703296 / 41108077155)
| root | fix | effect |
|---|---|---|
| The flush-time scan view: vanilla-routed items seen POST-move by the flush scans, PRE-move by vanilla's per-body scans (exact numbers: 1827 spawn y=-58.997 inside 971's window, post-body y=-58.84 outside) | L44 d01f631: the batch captures the PRE-body AABB (IB_PRE); the apply pass builds the VIEW (resync+force-vanilla, inactive excluded); the row path substitutes the PRE box + never window-skips + skips the home re-check for AFTER-scanner entries; the native path substitutes in the exact re-check; fastQuery refuses the native when a view entry's PRE box touches | items t=159→219, mixed t=152→158 |
| The row publish blackout: same-cell box updates returned UpsertEv::Same → no mark_dirty → the rows' packed x-intervals froze at the last New/Moved drain; the window false-skipped candidates whose CURRENT boxes overlapped (1088 skipped with a TWO-move-stale interval [33274,33535] vs window [33584,34869] — MSSR-proven) | L44c 0996e7f: UpsertEv::BoxChanged { cell } from the same-cell path; note_upsert marks dirty; the drain's publish rewrites the row. (The first L44c commit silently failed to patch the return — the MSSR dump of the follow-up run reproduced the stale interval verbatim; always verify a patch landed.) | mixed t=158→233, spread t=210→235 |

### Instrumentation built this session (committed to scripts/)
`ph_first_div.py` (the first PH/SR/LR divergence per leg pair), `pd_first_entity.py`
(the first diverging ENTITY via PD), `ptrace_cmp.py` (the PTRACE field diff of one
id across legs), `ms_forensics.py` / `ms_extract.py` (MS/MSQ/MSR/MSS/MSH/MSP/QB
merge-scan telemetry). Markers live behind run-baseline.sh's stream grep —
EVERY new marker must be added there (MSR, QB, MSS, MSP each bit once).

### The remaining divergences (head 0996e7f)
1. **items t=219 (6931/7892)**: 6931's scan went `path=mirror` — the L44 residual
   (the row refused AND viewTouches refused the native → the vanilla body at the
   flush, view-stale). Next: name the row refusal reason (row counters at the
   scan) and decide: serve the row for that shape or pre-stage the refusal.
2. **mixed t=233 (235/927), spread t=235 (3895/4142)**: the tc%40 resting-cadence
   scans (tc=80, mv=2) with n=0/1 — the same family, needs the MSSR/MSP dump at
   ct≈699/705 (the run 41110186252 window 210-240 covers it; the pairs are named).
3. **mobs t=275**: the push mirror pair (PD at 265-285 needed) — same flush-timing
   family on the push side (the L12 self writeback class), a separate hunt.

### Toolchain note (the sandbox reset)
cargo 1.99.1 (rustup minimal), Temurin JDK 25.0.5 (~/tools), the paper 26.3
mapped jar via fetch-paper.sh + one paperclip boot (work/papermerged/paper-26.3.jar
+ libraries symlink), `cargo check -p centities-module` compiles CeHooks.java via
build.rs (PAPER_JAR default path). paper-api 26.3.build.218-stable is PRUNED from
repo.papermc.io — the local pom pins it; CI still resolves it from cache.

## Session addendum 9 (2026-10-30/31): the refusal named, the push family proven, L46/L47

### The sandbox reset and the toolchain rebuild
The session resumed into a recycled sandbox: rustup/Temurin/paper-jar gone,
`gh` missing (the API + the git remote token carried everything). Rebuilt:
cargo 1.99.1 (rustup minimal), Temurin JDK 25.0.5 (~/tools/jdk25),
paper-26.3-231 via fetch-paper.sh + `paperclip.patchonly` boot, the
mojang-mapped jar at work/paper-merged/paper-26.3.jar + `libraries` symlink
(fastutil lives there; without it javac fails on it.unimi.dsi.fastutil).
`PAPER_JAR=... cargo check -p centities-module` compiles CeHooks.java locally.

### The ct=×3 key (why the old runs DID cover the divergence ticks)
`MS_WINDOW = PH_DUMP × 3 levels ± 15` (parseMsWindow): CUR_TICK counts
flushes across ALL loaded levels, so the markers printed at CUR_TICK
616..733 ARE the server ticks 205..244 — the divergence window. "ct=" in
ms_forensics prints the CUR_TICK (server tick ≈ ct/3). The old belief
that the markers missed t=219/233/235 was wrong; the window input is in
SERVER ticks and covers both the PD gate and the scan markers at once.

### items t=219 (6931/7892) — the refusal named, mechanism confirmed
L45 instrumented every fastQuery/rowQuery refusal point with fastRefuse
and the mirror MSQ line now prints `refuse=`. The diag run (41119838068,
ph_dump=210-285, trace_id=235):
- 6931's scan at ct=655: `path=mirror rawN=0 ids=[] refuse=view` — the
  row path refused (reason was OVERWRITTEN by the native leg of the
  chain — fixed in the follow-up: the chain now prints e.g.
  `pending+view`), the native refused on viewTouches (7892's PRE spawn
  box touches), the mirror ran the vanilla body at flush time on the
  view-stale world → the merge vanilla did (OFF: 6931 absorbed by the
  fresh 7892) never happened. The exact L44 residual.
- Also: msqIds() emitted `ids=]` for empty scans — the ms_forensics
  regex (requires `[`) silently dropped EVERY mirror line. Fixed both
  sides ("[]" + optional ids in the regex).

### mixed t=233 (235/927) — the PTRACE verdict: an EXTRA PUSH
trace_id=235 across both legs: entity 235 is an **ARMOR_STAND** frozen at
(-0.5, -59.75, 0.5) — resting ON the item pile (box bottom exactly
touching the pile's top at -59.75; onG=0 — entity-supported, cramming
equilibrium). t=228..232 both legs identical, all-zero motion.
**t=233: ON's stand gains vx≈-2.6e-3, vz≈+1.4e-2 while OFF stays zero**;
the velocity persists (frozen at the t=236 value — the pile re-absorbs
it). The kernel accepted a push pair vanilla's in-phase scan excludes.
The kernel's compare is exact (pushcore.rs strict f64, margin only
widens the CELL walk), so the culprit is the mirror state: a stale
box6/position/flag on the candidate (or the pusher) at accept time.
- spread t=235 (3895/4142): the same one-tick family (4142's hash froze
  at a DIFFERENT resting value — one displaced rest).
- mobs t=275: first diverger zombie 1069, cascade 78/916 — same class.
- The pairs were re-confirmed on the fresh run's PD (210-285).

### L46 (doc 41 move 2) — the shared scratch answer list
mergeScanNative consumed its candidate list synchronously but every scan
allocated a fresh ArrayList (~32k/tick on items-100k). FQ_SCRATCH is
server-thread confined, depth-guarded (a reentrant query inside the
consume loop allocates; getEntities keeps allocating), knob
CE_FQ_SCRATCH=0. Core tests 43/43 green.

### L47 — the push forensics + the refuse chain
- `KPH t= self= tc= n= box=[...] id:dx,dz...` in the kernel-accepted
  push path at PD ticks (added to run-baseline.sh's stream grep): the
  pusher identity + live box + the exact pair set with forces. The OFF
  leg's PD/PTRACE supplies the vanilla outcome; the diff names the
  culprit pair.
- fastRefuse keeps the ROW reason on the native view refusal (the
  chain form `pending+view`).

### The fresh mixed-150k profile (lean run 41112622574, bd5eef6)
mod p50 **319.8 ms** (doc-41 era: 412.2 — L44c/L44d/L45 helped the A/B
legs). Top module buckets (35658 samples): the tracker cluster
newTrackerTick$ 3.90 + trackerDispatch 1.58 + SynchedEntityData 2.78 +
getItem 1.44 + sendChanges 1.07 + sendDirtyEntityData 0.82 ≈ **11.6%**,
Unsafe.getObject 3.24, tickListForEach 2.98, setTicksFrozen$ 2.42,
itemTickFlush 2.28, ttlFastPath 2.51, **isViewAfter 0.91 (NEW LEVER: the
L44 view membership check runs twice per row candidate — gate on the
view being empty)**, tickEffectsNoOp 0.77. AABB.intersects fell to
0.42 (the doc-41 8.3% bucket mostly went with L44c's row publish).

### The next moves (in order)
1. The KPH run names the mixed/spread/mobs culprit pair → fix the extra
   push (mirror staleness or the PF_PUSHABLE refresh path).
2. The items view-union: serve `rows ∪ view-entries` (the view entries'
   PRE boxes ride B_SY/B_SEQ order stamps) so the merge scan never falls
   to the view-stale mirror for the pending+view shape.
3. The isViewAfter hoist (a viewCount != 0 gate).

### Session addendum 10 (2026-10-31): L49 lands — items 219 → 340; L50 targets the push family

#### L49 (ad6b33f, VERIFIED by the gate run's PH map)
The row path now SERVES the covered pending-fresh entries instead of
refusing (the exactness rule: KIDX <= MS_KIDX → the live box;
KIDX > MS_KIDX → the batch box, exact only when live == batch box —
untouched since collect; a moved entry still refuses). The gate run's
ph_first_div: **items first divergence t=219 → t=340** (+121 ticks);
mobs 275 / mixed 233 / spread 235 unchanged (the push family — L50's
target). The refuse chain confirmed the mechanism on the KPH run:
`refuse=pending+view` (the row refusal was pending; the native's
viewTouches stacked after it).

#### The push family NAMED (the KPH run 41123470972)
- mixed: zombie 927 → stand 235, first push ct=696 (t=232), force
  -0.0026 == the stand's ON-leg vx at t=233 (the PTRACE match);
  repeated at 699/702/705 while vanilla stayed zero.
- spread: zombie 3895 (tc=18) → 4142 at ct=702.
- mobs: zombie 78 → 1069 at ct=822 (t=274; the PD divergence at 275).
The kernel's box compare is exact (strict f64, margin only widens the
CELL walk) and the positions/boxes are fresh (travel precedes
pushEntities in aiStep — decompile 3736/3752; the pending probes drain
before nativePush) — the culprit was the CAP STATE.

#### L50 (ded1978): the cap replay reads the REAL numCollisions
Vanilla-routed pushEntities bodies (the density-gate refusals, the
decline paths) increment candidates' REAL numCollisions without
touching pc.ncol — the mirror lagged, the reset replay
(max(0, ncol - maxEC)) started below vanilla's cap, and the kernel
pushed pairs vanilla's `break` had already excluded. The Java guard
reads the REAL field (one Unsafe read at dispatch) and passes it to
nativePush; the replay uses it verbatim. JNI (IIIIIIIII)J.
L50b: the pushcore tests (the e2e gate caught what the lib-only check
missed — run cargo check/test --all-targets locally). L50c: the
natives-parity manifest tracks the 9-int descriptor.
VERDICT: pending — the ded1978/464c18e parity runs name the next
first-divergence map (ph_first_div on their logs; the items-340 pair
via the 8e741a8 dispatch with ph_dump=330-350).

#### The fresh lean profiles (41112622574, bd5eef6)
- mixed-150k mod p50 319.8 ms; items-100k mod p50 29.1 (−50.6% vs
  vanilla 58.9); fastQuery GONE from both top-15s (was 13.96% on
  items). The items top is now the tracker (12.04) + the flush
  (12.54) + isViewAfter 3.13 (the L48 hoist's target).
- mobs-25k stays a flat long tail (Unsafe 3.62 top).
- The lean numbers are NOT comparable to the baseline.yml table
  (different run parameters) — use them for SHAPE, not for deltas.

## Session addendum 11 (2026-10-31): L51 — the push family NAMED and FIXED (ArmorStand); items-340 ground truth (the double gravity)

### The extra-push family: the candidate-side predicate missed the OVERRIDES
`EntitySelector.pushableBy(pusher)` = `input.isPushable() && input.canCollideWithBukkit(entity)
&& entity.canCollideWithBukkit(input)`; `LivingEntity.isPushable()` = `isCollidable(cfg)` —
**VIRTUAL**, and 26.3 ArmorStand OVERRIDES it to `return false` unconditionally (empty doPush,
pushEntities = rideable-minecart lookups only, `MARKER_DIMENSIONS = 0x0`). The scenario mix
spawns `pctStands` MARKER stands in EVERY scenario, so the push-family candidates were stands
all along: mixed 235, spread 4142, mobs 1069 (the addendum-9/10 "zombie" naming was the PUSHER
side; the pushed candidate is the stand). The kernel's L12 fold (`pushFlags`) inlined the
LivingEntity BODY of isCollidable — field-for-field exact for plain LivingEntities, blind to
every override — so stands entered the pushable-only index with a stale PRE-marker box (the
0x0 box never arrives as a cell change) and the kernel pushed pairs vanilla never sees:
mixed zombie 927 -> stand 235 at ct=696 force -0.0026 (the PTRACE-matched extra vx),
repeats 699/702/705; spread 3895 -> 4142 ct=702; mobs 78 -> 1069 ct=822. L50's cap replay was
real but not this bug (the post-L50 KPH run 41132152574 still pushed 927 -> 235).

**L51 (5992c32)**: `pushFlags` PF_PUSHABLE = the VIRTUAL `le.isPushable()` — one call, exact
for every subclass override (AbstractHorse/Parrot also override); the L38 collide fast path
already made the same virtual call. Stands exit the pushable-only index in BOTH directions.

### items-340: the PTRACE ground truth (run 41133998643, trace_id=2)
Entity 2 (an old pile ITEM): position x/y/z BIT-IDENTICAL across the legs for the whole
window; the ONLY diverging field is **vy**: t=339 both `-0x1.47ae147ae147bp-5` (-0.0808);
t=340 ON = the same mantissa at p-**4** (-0.1616 = exactly 2x); t=342 OFF settles to vy=0.0
(the ground clamp on its (tc+id)%4==0 move tick) while ON still -0.1616; t=343 SAME (ON's
move tick caught up). So ON applied the item's gravity step TWICE at the batch tick (the
vanilla tick AND the flush/fast-path), the extra vy never integrates into position (the
shortcut ticks skip move(); the ground clamp hides the rest), and the legs re-converge after
the next bounce tick — except the per-tick PH sees the vy bits and fails at t=340, with the
merge-removal gap (5..22 entities, the n-gap) riding the same flush-timing skew. The m
history (2,3,43,47: ON changes at 340, OFF at 341/342, both meet at 342) is the same wave.
L52 target: the item flush/fast-path must not run a gravity/physics step for entities
vanilla also ticks this tick (double-tick at the spawn-batch flush).

### L51 verdict (the 5992c32 gate run): the push family is DEAD
mixed t=233 -> t=340 (+107), spread t=235 -> t=259 (+24). The remaining first
divergences (items 340, mixed 340, mobs 275, spread 259) all sit at the
tc=200 activation boundary of their item cohorts — the L52 family below.

### L52 (this commit): the MODE-1 dispatch gate read the tickCount PRE-phase
The dispatch weave OWNS the dispatcher for MODE-1 items (tickNonPassenger
suppressed — commonTick has NOT run; the Rust core mirrors the ++ itself), so
the gate sees the PRE field and must pass preIncrement=true — the rest branch
(MODE-2) already did. The stale preIncrement=false (correct in the t=159-era
weave, which did NOT own the dispatcher) read 199 where vanilla read 200:
at the tc<200 activation-immunity end the engine granted ONE extra ACTIVE
tick (gravity) — PTRACE item 2 (run 41133998643, trace_id=2): position
bit-identical all window, vy -0.04 -> -0.08 (exactly one 0.04 step) at
t=340, the wakeup settle 343 vs vanilla 342, tc bit-identical (196..207),
activatedTick MIN_VALUE both legs (no sweep stamps — the wakeup phase is
cur-driven and identical). The n-gap growth (5..22 entities) rode the same
one-tick skew through the merge-removal cadence. The +1 changes behavior
ONLY at the tc=200 crossing (young items return active under either phase),
so the t=159 fix surface is untouched.

### L52 verdict (the fed6f56 gate run): the tc=200 crossing moved one cohort
items t=340 -> t=353, mixed t=340 -> t=353: +13 ticks == EXACTLY the next
item cohort's crossing (153+200=353) — the L52 phase fix is real and the
divergence is a CHAIN of per-cohort boundary crossings. mobs 275 /
spread 259 unchanged (pure-zombie stale-box class / a separate family;
the trace_id=1069 dispatch is in flight). NEXT LAYER at the crossing:
for ids with (tc+id)%4==0 (id == 2 mod 4) the crossing tick tc=200 ALSO
hits the merge cadence (200%40==0) — an active-tick merge scan running
at the activation boundary; the 353 pair needs the PD window 348-360 to
name the cohort's diverging ids and the changed field (count vs vy).

## Session addendum 12 (2026-10-31): L53 — the items-353 wave NAMED and FIXED (the inactive tick's absolute on-ground bit)

### The dispatch map reproduced (the e71dcc7 PD runs 41186373869 + 41188331063)
Both dispatches (ph_dump=348-360/trace 1069, then 295-365/trace 324) land
the SAME first-divergence map: mobs t=301, items t=353, mixed t=320,
spread t=278 — bit-identical across runs (the determinism belt holds).
The t=353 wave: the PD diff shows 533 items (items), 311 (mixed), 201
(spread) diverging AT ONE TICK with no removals (missing=0); the T389
snapshot deltas are vx/vz-only at 1e-15..1e-42. The L52 verdict's
"id==2 mod 4" guess is WRONG — all four residues diverge.

### The crossing semantics (the 26.3 decomp ground truth)
- tickNonPassenger: commonTick (tickCount++) -> checkIfActive -> tick()
  or inactiveTick(). The activation port's preIncrement=true phase is
  CORRECT (L52).
- The global wakeup: (cur - activatedTick - 1) % 20 == 0; for a
  never-stamped entity (activatedTick = -2^31) that fires when
  cur % 20 == 13 — t=353 is the FIRST such tick for the first cohort
  (spawned t=153, the tc=200 crossing at the same tick).
- The item move-skip rotation lives INSIDE tick(): gravity runs
  unconditionally, but move()+friction+bounce only when
  (tc+id)%4==0 || !onGround || horiz>1e-5. A skip tick leaves vy=-0.04
  residue that the inactive ticks then PERSIST (inactiveTick has no
  physics).

### The wave mechanism (the FTICK store trace, CE_RTRACE_ID)
tick_item's ON_GROUND outcome bit is ABSOLUTE (the final state) — the
L8.2 commit applies it verbatim: entry.on_ground = bits & B_ON_GROUND.
tick_item_inactive ran no physics and set NO bit, so EVERY inactive
tick clobbered the store's on_ground to FALSE. Young items (tc<200)
tick active every tick and always carry the absolute bit — the bug was
invisible; at the tc=200 crossings 19/20 ticks go inactive and the next
active tick's move gate (!st.on_ground) opened and ran a SPURIOUS move
(gravity+collide+FRICTION, a x0.588 step on vx/vz) that vanilla's skip
tick never does. The wave is density-coupled because the friction step
is only visible on items with dx/dz != 0 (the merge-scattered pile).

**L53 (e226a3a): tick_item_inactive sets the same absolute bit.** The
local A/B at CI density (SCALE=12000, AREA=115): the T389 snapshot was
801 diverging items before, bit-identical after.

### The tooling that closed it (all default-off, riding main)
- DTRACE (PARITY_TRACE_ID): the per-tick itemDispatch decision — the
  path taken + the port's activation verdict + the tc/at the gate saw.
  Proved the activation port EXACT tick-for-tick at the crossing.
- FTICK (CE_RTRACE_ID): the flush store view (flags, st.tick_count,
  st.on_ground) — named the clobber.
- MSIB (CE_FQ_IBOX=1 + core EntityIndex::box_of): the marginExtra
  reject sample's Rust side (the index box vs the live box).
- The local parity loop: one scenario pair in ~8 min (SCALE 6000-12000,
  MEASURE_TICKS=50), scripts/run-baseline.sh with MODULE=on + the
  pre-staged paperboot jar/libraries.

### The remaining families (the next layers)
1. **mobs t=301 / spread t=278 / mixed t=320 — the inactivated-zombie
   stale box.** The trace-823 root: the OFF zombie moved MORE at the
   crossing tick (the young neighbors push it; vanilla reads the LIVE
   box). The just-inactivated zombies stop running bodies, so their
   probe batches stop; the kernel's push mirror keeps the LAST-ACTIVE
   box — the pushes diverge exactly when a cohort crosses tc=200.
   Fix surface: a freshness story for the inactivated entities' mirror
   (probe at the walk level, or refuse the stale candidates).
2. **The items belt dust: t=416+ (the L7 256-belt era), 2 items at
   1e-21 vx/vz.** Post-fix local pair: T389 bit-identical, the belt
   re-ingest era still splits a pair of items. Same hunt shape: the
   FTICK/PD at 409-420.

### The -90% grind (the b676709 baseline, e424b2a)
items -54.6 / mobs -50.0 / mixed -54.7 / spread -31.4 (p50, run variance
+-4pp). The doc-41 ranked moves stand; the parity floors above gate them.

## Session addendum 13 (2026-10-31): the stale-box family RE-ROOTED — the push integration timing, the 26.3 ground truth, the TL order stream

### The PD windows reproduced the dispatch map bit-identically (runs 41214287627 / 41214289587, e226a3a+ed85916)
ph_dump 270-325: mobs FIRST divergence t=301 (ONE entity: 823), spread t=278
(4046), mixed t=320 (3461); items CLEAN 270-325. Same map as the e71dcc7
dispatch — the determinism belt holds on the L53-era main.

### The PTRACE verdict (run 41216108186, trace_id=823; run 41217812956, trace_id=556)
mobs 823 (a STATIC isolated zombie, tc=93.., young => ALWAYS active via the
tc<200 branch): t=300 bit-identical (x=15.75, vx=0). At t=301:
- OFF: x UNMOVED (15.75), vx = -0.038078 PENDING — the push landed DURING
  tick 301 AFTER 823's body (its travel never integrated it this tick).
- ON: x MOVED (15.7043), vx = -0.020845 INTEGRATED (-0.038 x the on-ground
  friction, exactly one travel) — the push landed BEFORE 823's body ran.
The same force (-0.038073, the kernel pair 556->823 at CUR 900 = plugin
301, KPH box center 16.0798 => dx_raw 0.58) landed and INTEGRATED in ON.
556's own PTRACE (both legs bit-identical through t=301, x=16.079 at its
turn-301 position) proves 556 was at the pusher position in BOTH legs at
t=301. spread 4046 / mixed 3461: the same static-isolated-zombie profile.

### The 26.3 ground truth (mapped jar javap, /tmp re-staged this session)
- pushEntities has EXACTLY ONE call site: LivingEntity.aiStep offset 839.
  INACTIVE entities never run aiStep => an inactivated zombie never ACTS
  (it is a receiver only). The pushcore doc's "all of them run
  pushEntities each tick" is wrong for inactivated entities (harmless:
  the kernel's pushDispatch is woven INTO the vanilla body).
- Entity.push(Entity) decoded: f = min(1, 1/sqrt(absMax(dx,dz))) — for
  every box-overlapping pair (d<1) the force magnitude is EXACTLY 0.05 x
  the normalized direction component; the self gets the opposite force
  when pushable and not a vehicle. The kernel's pushcore math MATCHES
  (the earlier -0.038 decode was the normalized component, not a bug).
- LivingEntity.pushEntities decoded: the nextInt(4) cramming draw happens
  ONLY when list.size() > cramming-1 (ServerLevel + cramming>0 gates);
  an EMPTY list early-returns BEFORE the numCollisions reset. The
  kernel's cramming flag gates the draw the same way — the draw count
  matches iff the candidate count matches.
- checkIfActive decoded: tc<200 => active (the crossings' mask);
  otherwise the (currentTick - activatedTick - 1) % 20 == 0 wakeup runs
  checkEntityImmunities; immunity = !onGround || horizontalDistanceSqr >
  9.999999747378752e-6 => a 100-tick window (plus fire/water/climb/
  target/... branches).
- ServerLevel.tick: activateEntities(level) ONCE per tick before
  entityTickList.forEach(guardEntityTick -> tickNonPassenger); the
  stamp scan is one getEntities(null, maxBB) per player — routed through
  the module's query paths in the ON leg (row path refuses wide boxes ->
  the flush-merged native path — fresh by construction).
- ItemEntity.tick: the merge scan runs AFTER the move (offset 481), the
  cadence rate = 2 if the position changed else 40.
- EntityTickList = IteratorSafeOrderedReferenceSet: insertion order;
  remove() NULLS the slot (no swap) and defrags ONLY when no iterator is
  active and frag>=0.2; defrag compacts IN ORDER — the iteration order is
  INVARIANT to removal timing. The tick order cannot diverge from merge
  removals (the earlier swap-remove theory is DEAD).

### The open contradiction (what the TL stream must name)
The tick order should be immutable, yet the OFF leg integrated the SAME
push one turn later than ON. Either (a) the OFF-leg pusher is a different
entity ticking after 823 while ON's pusher (556) ticks before it — i.e.
the KERNEL pushed a pair vanilla's candidate list did NOT contain at
556's turn (the vanilla list is built from the LIVE boxes — 556's box
[15.779,16.379] vs 823's [15.2,15.8] overlaps by 0.021 — vanilla SHOULD
have pushed too), or (b) the tick order really diverged. The TL dump
(ce.tl.trace=1, run dispatched on e4227e3) names which: the first tick
where the TL id sequences differ, and the entities whose relative order
flipped. NOTE: the PD/PTRACE snapshots ride ServerTickEndEvent — the
"t=301" sample is the END-of-tick state (the pending-vs-integrated
delta at the same tick is the order proof).

### The tooling added (all default-off)
- TL (ce.tl.trace=1): the tick-list order dump at PD ticks, both legs
  (BaselinePlugin, read-only reflection, 10 ids/line).
- pd_window.py: the per-leg PD diff in a window (the first divergent
  entity per scenario).
- ptrace_win.py: the PTRACE diff per leg pair around a window.

## Session addendum 14 (2026-10-31): the belt dust LOCALIZED — one extra friction step at t=413, a 53-id cohort, a closed move gate

### The belt-dust hunt run (41221146878, 9ca0415; knobs itemTick=1 mobAi=0
### merge=1 TRACE_ID=1408 PH_DUMP=405-425)
The gate failure map: mobs T382/T632/final FAIL (yaw 5.34 at T382 — the
push family), items T389 PASS (bit-identical — L53 holds), items
T639/final FAIL, mixed/spread all FAIL. The items T639 diff: 1214 items
drifted, vx/vz ONLY (x/y/z/yaw/vy all 0.0), worst deltas ~3.2e-20 —
DUST. And the drift is a RATIO: every drifted id's off/on vx ratio is
1.7006801 == 1/0.588 — EXACTLY one on-ground friction multiply. The ON
leg is BEHIND by one 0.588 step: OFF carried one MORE friction multiply
than ON — no, the PTRACE proves the opposite direction (below).

### The PTRACE split (scripts/ptrace_split.py, item 1408, both legs)
- t=412: vx bit-identical (-2.921755e-17 both), onG=1 both, tc=258.
- t=413: ON vx = -1.717992e-17 (ONE 0.588 step APPLIED), OFF unchanged.
  tc=259. The move rotation for id 1408 (1408%4==0) needs tc%4==0 —
  259%4=3: NEITHER leg's rotation tick. ON ran a SPURIOUS move.
- t=414: OFF applies ITS rotation friction (tc=260%4==0) — catches up to
  ON's 413 value; ON applies the rotation too (-1.010e-17 both... ON
  stays one step ahead: the ratio pins at exactly 1.7006801 forever).
- The cadences RE-SYNC after the single spurious step (414, 418 both
  legs) — the divergence is ONE extra multiply, not a phase shift: the
  store's on_ground re-healed on the next rotation tick (the L53
  absolute-bit write from the rotation's own move outcome).

### The PD diff (scripts/pd_pair_diff.py, per-entity hashes, both legs)
- t=410/411/412: 0 diverged of 16498 common (bit-identical).
- t=413: 53 diverged — ALL id % 4 == 0, a CONTIGUOUS cohort
  (1408, 1412, ... 1476, one belt segment), no removals (only_off=0,
  only_on=0). 53 stays 53 through 415+; later waves (the T639 1214)
  must hit the other id-residue classes at later ticks — the same
  one-tick anchor repeats per cohort.

### The mechanism (pending the store view)
The spurious move needs the tick_item gate to open OUTSIDE the rotation:
`(tc+id)%4==0 || !on_ground || horiz>1e-5` — onG=1 and horiz~1e-17 at
413, so the KERNEL's st.on_ground said FALSE at the t=413 gate for the
53. The t=412→413 boundary wrote on_ground=0 for one contiguous belt
segment. The L53 fix makes tick_item_inactive SET the absolute bit —
a preserve; if the FLUSH upsert clobbered the bit to 0 between the
ticks, the inactive preserve carries the clobber and the next active
gate opens. The FTICK/RTICK view (CE_RTRACE_ID — the store's
tc/on_ground/delta AT the body's gate, plus the flush batch's flags)
names the writer; run 41234855357's TL stream did NOT fire either
(-Dce.tl.trace=1 never reached onEnable despite the step env dump —
the property path is untrusted; the CE_TL_TRACE env arm landed in
ce5a471 with the workflow extra_env input).

### Tooling
- scripts/ptrace_split.py: the per-tick PTRACE diff across the two
  legs' line ranges (the first vx/vz split tick + the ratio table).
- scripts/pd_pair_diff.py: the per-entity PD hash diff for given ticks
  (the diverging id set + the removal check).
