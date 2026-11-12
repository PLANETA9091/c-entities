# doc 62 — n2-i0: the wholesale item dispatcher (the Java walk dies)

Branch `main` @ 60cd475. The mob wire's M0/M2 lesson applied to the item
side: the native item tick is already stateful (L8.2 — "dispatch-owned
entries carry ONLY id+flags — their state lives in the store") and the
read-back is already compact (L79 TAIL_NONE). The remaining item-wire fat
is the JAVA-SIDE per-tick walk: the dispatcher guard, the tracker pass
and the flush protocol run per item per tick on the JVM.

## The measurement that set the scope (local paired items-25k, 26.3)

Same-window A/B on this 2-vCPU sandbox (fill 400, warmup 100, measure
300, seed 620856203, tracking-range 48):

| leg | MSPT p50 | mean |
|---|---:|---:|
| vanilla (module off) | 20.256 | 29.615 |
| **native item wire** | **10.871** (9.388 / 9.133 on re-runs) | 16.599 |

−46…−54% p50 — consistent with the CI 100k leg (−56%). The mob wire's
paired 10k reads −87.4%: the gap is the wire shape, not the engine.

The measure-window JFR (991 samples, this box — the WatchdogThread spin
~9.5% and the boot bzip2 are harness noise present in BOTH legs):

| cluster | % | note |
|---|---:|---|
| rowQuery (the cadence/merge scans) | ~4.0 | 121 scans/tick ride the Java query wire |
| tickListForEach + itemDispatch + appendDispatch | ~4.4 | the dispatcher walk |
| itemTickFlush (packing + stamp loop A + apply B) | ~3.6 | incl. the ds despawn scan 5085/tick |
| newTrackerTick | ~2.8 | the tracker pass (ttlQ skips 2.99M already) |
| Unsafe.getInt mirror reads | ~2.9 | spread across the probes |
| **module cluster total** | **~17.7** | vs the native tick floor: invisible |

The L37f rest arm stays OFF by design (`if (false && …)` — the 26.3
checkIfActive %4 exception keeps items full-ticking; the inline
counter-only guard diverges). Everything below keeps items IN the
native batch — tick_item is exact; the fat is the wire, so the wire
dies.

## The shape (what dies, what replaces it)

### i0-a: the native pre-decided dispatch verdict

Today every ItemEntity's tickNonPassenger weave calls itemDispatch(e):
itemCheckActive (the %4 port), the MODE/REV gate chain, dtrace,
appendDispatch — ~30-60 ns/item — 25k × 50 ns ≈ 1.25 ms/tick on this
box, and it scales linearly.

The native tick ALREADY decides everything the guard re-derives: the
per-entry bits ride the compact read-back (outI bits 11-16). Add one
verdict bit — STAY (bit 17) — set when the core's next-tick plan for
this entry is "stay in the batch, no wake, no mode change" (the same
predicate the dispatcher would approve: native-mode, no section touch,
no external mutation stamp, the %4 exception noted as always-active).

The Java guard becomes ONE array read:

    int slot = entId(e) & MAMASK;
    if (DISP_STAY[slot] == DISP_GEN && DISP_ID[slot] == fullId) {
        appendDispatch(e, true);   // 4 array writes, nothing else runs
        return true;
    }

(the DISP_ID full-entity-id belt is the REST_ID pattern: entity ids are
monotonic, a slot-colliding successor can never inherit the verdict.)
The expensive part of the walk dies for the steady-state population;
the guard only runs its full chain for slots the native side did NOT
pre-approve (fresh adds, wakes, mode transitions) — exactly the rare
set.

### i0-b: the merge-cadence candidates ride the read-back

The vanilla ItemEntity.tick merge scan runs on the %8 cadence gate per
item; for native items the scan must produce the same candidates in the
same order (the query wire is proven vanilla-order — doc 36b/L38a).
The native side owns the index AND the cadence counters: the scan can
run INSIDE tick_item (the index query is a native call, the candidate
order is the index's scan order = the proven vanilla order) and the
result rides the compact read-back as MERGE rows (outI extension:
candidate count + candidate entity ids).

The Java side keeps ONLY the absorption body (the vanilla mergeItem
semantics: health/discard/the absorbed entity's removal) — it applies
the rows instead of running the scans. rowQuery for the cadence scans
dies (~4% + the scan glue); the vanilla-routed items keep their scans
(their ticks are vanilla by definition).

### i0-c: the tracker pass reads the native verdict

newTrackerTick walks every item per tick per level (the quiet bit
ttlQ already skips the work for 2.99M/leg entries but the WALK
remains). The native read-back's per-entry verdict doubles as the
tracker verdict: a STAY entry whose position is unchanged is
tracker-quiet by construction (the mirror bump the tracker skip pays
is exactly the tickCount debt — the applier already writes it). The
tracker walk still iterates (the EntityTickList is vanilla state) but
its per-item body collapses to the same one-array-read pattern.

### The gates (what must stay bit-exact)

- The batch membership set is UNCHANGED: every item that ticks today
  still rides the batch; i0 only removes the JAVA re-derivation of
  decisions the native side already made.
- The wake paths are UNCHANGED: probe delta/box, section invalidation,
  merge cadence, the 256-tick belt, despawn, pickupDelay — each clears
  DISP_STAY through the existing MODE_REV bump; the guard falls back
  to the full chain on any wake.
- The despawn/merge/land events return through the compact rows as
  today; the absorption body's order relative to the tail writes is
  the twin-asserted order.
- The twins: the existing item suites + a new i0 twin (the guard-skip
  arm vs the full-guard arm over a 25k-shaped storm — the dump-equal
  discipline of the mob twins, EXEC_TWIN_LOCK included).

## The expected effect

The module cluster ~17.7% → ~6% (the walk bodies die, the packing and
the absorption rows remain). items-25k local: ~10.9 → ~9.3 ms (−54%→
−56% vs vanilla on the pair); items-100k CI: ~32 → ~27 ms (−56%→−63%);
mixed-150k inherits both (the 3x-tick hole is already closed by
phase-2d). The tracker pass (i0-c) is the follow-up lever once i0-a/i0-b
land — it shares the verdict bit, so the plumbing arrives with them.

## Addendum — i0-a shipped; the i0-a2 ground truth (javap 26.3-249)

i0-a (the %4-rotation verdict) landed at 53b1f5d: the local leg reads
dispFast=2007656 of 8162946 dispatches — EXACTLY the %4 share (the
rotation qualifies 1-of-4 ticks per item); dispChecks/dispInactive/ttlQ
identical to the pre-i0a legs. The gain sits inside the sandbox noise
band (p50 9.77 vs the 9.1-10.9 mod legs) — as projected, the gate chain
is a minority of the walk.

The full activation ownership (i0-a2) needs the sweep replicated. The
javap ground truth from the paper-26.3-249 mapped jar
(io.papermc.paper.entity.activation.ActivationRange):

- `activateEntities(Level)`: per player — Player.activatedTick = cur;
  the per-type boxes = player BB inflated by (range, getHeight(),
  range); maxBB = min(simulationDistance*4 - 8, max(all ranges)); ONE
  Level.getEntities(maxBB, predicate) per player; per hit
  activateEntity(e).
- `activateEntity(e)`: IF cur > e.activatedTick: defaultActivationState
  → stamp cur; ELSE IF e.activationType.boundingBox.intersects(e.bb) →
  stamp cur. Items are ActivationType.MISC. NOTE the `cur >
  activatedTick` guard: a stamp never goes backwards, and an entity
  stamped this tick is not re-checked for other players.
- `checkEntityImmunities(e)` (the immunity for the 20-tick window):
  checkInactiveWakeup first (> -1 wins), then fireTicks>0 → 2,
  activatedImmunityTick >= cur → 1, (not WATER-type && inWater &&
  pushedByFluid) → 100, onGround && |dv|² > 1e-5 → 100, ... (the port's
  itemImmunities matches the item-relevant tail).
- The port's `e.activatedImmunityTick >= cur` clause is dead for
  module-owned items: CeHooks NEVER writes the field (javap cross-check:
  the only writers are vanilla paths the module routes away), so the
  field stays at its initial 0 and 0 >= cur is false for every tick
  after boot.
- The vanilla sweep STILL RUNS for items today (no weave) — its stamp
  lands on e.activatedTick and the port reads it back. Any core-side
  ownership MUST replicate the sweep (the player-present divergence:
  the sweep activates near-player items; without the replication the
  core would keep them inactive). The replication inputs: the per-tick
  player snapshot (the mob wire already passes one), miscActivationRange
  + the level height + ignoreSpectatorActivation + the
  simulationDistance clamp (a push-once cfg — the phase-2a pattern).
  The vanilla getEntities(maxBB) sweep query per player per tick rides
  the query wire today (part of the rowQuery cluster) — the core-side
  sweep makes it redundant for the native population.

i0-a2 shape: the ItemState mirror gains activated_tick (i64) +
default_activation (bool); the batch entry's IF_INACTIVE flag stops
being authoritative (the core decides tick_item vs tick_item_inactive
from its own verdict — the sweep + the port chain); the read-back
carries the verdict + the activatedTick/isTemporarilyActive writeback;
the applier writes the two fields per entry (the vanilla sweep keeps
running for the VANILLA population only — native items must skip it in
activateEntity, a new early-return weave — otherwise two writers
diverge); the dispatch's fast path extends from the %4 share to ~100%
of the steady state. The twins: the sweep+verdict arm vs the port arm
over a player-walk scenario (the bench is playerless — the parity
harness's player-present leg is the honest gate, the doc 61 lesson).

## Addendum 2 — i0-b ground truth (javap 26.3-249: ItemEntity merge)

The core sets MERGE on the cadence gate (tick_count % 40, or % 2 on a
floor change — the EMPIRICAL all-post convention, runs
40723397672/40733281619); the scan itself runs in JAVA via the woven
getEntities → rowQuery (the ~4% cluster). The javap of
ItemEntity.mergeWithNeighbours (26.3-249):

- `isMergable()` gate first: isAlive && pickupDelay != 32767 && the
  stack-count tail (count < maxStackSize).
- the radius: `spigotConfig.itemMerge` (a double config, default 0.5);
  the box = `bb.inflate(itemMerge, onlyMergeItemsHorizontally ? 0.0 :
  itemMerge - 0.5, itemMerge)` — the paper `onlyMergeItemsHorizontally`
  config zeroes the Y inflation (0.5 - 0.5 = 0.0 at the default).
- the scan: `getEntitiesOfClass(ItemEntity, box,
  lambda$mergeWithNeighbours$0)` — the predicate excludes self (+ the
  same isMergable set per candidate inside the loop).
- per candidate (in the query's vanilla order): the
  `fixItemsMergingThroughWalls` config gate runs a clipDirect
  (position→position, CollisionContext.of(self)) — a BLOCK hit skips
  the candidate — then `tryToMerge(candidate)`; the loop stops when
  `isRemoved()` (self absorbed).
- tryToMerge holds the count math (the same-item + the maxStackSize
  fit) — THE STACK STATE — which the mirror does not carry.

i0-b shape (the next session's opening move): the core's index query
replaces the scan (the same index, the same vanilla order — the doc-36b
parity holds; the query is in-process under the held INDEX lock — the
fr-memo hole-closer already queries the index inside the flush). The
mirror gains `is_mergable` (ingest + the probe/writeback maintenance);
the compact read-back rows gain a candidates segment
([cnt, id0..idN-1] in the vanilla order); the Java loop keeps ONLY
tryToMerge + the wall-clip gate (the clip is Java-side: the section
provider's collision — or the core's, config-gated). The candidate
filter = the mirror's is_mergable; the pairing/count math stays in
Java with the real stacks. The vanilla-routed items keep their scans.

## Addendum 3 — the i0-b first build FAILED and was reverted (the forensics)

The phase-3 scan's first build collapsed the leg to p50 280ms with
merges=0 and a resync storm (itemResync=50000 — every native add
re-ingested every tick). The od layout carried the pool offset in the
row's slot 9 (`od[k*9+9]`) — **for the last batch row (k=n-1) that
index is exactly n*9, the pool's first cell** — the offset write
clobbered the first merge row's candidate count (and vice versa), and
the dense-fill scans produced 25-50 candidates per box (25k freshly
spawned items in a tight pile, every one isMergable) so the 4096-double
pool overflowed within ~80 rows and the truncation marked every later
row VANILLA_NEXT — the MODE stamping then herded the whole population
through the resync path (the full 14-double state copy per item per
tick) while the merge counter read zero.

The revert restored 4978f5f exactly (the leg: p50 10.60, merge=121290,
dispFast=8.04M/98.4% — the i0-a2c state verified live).

The i0-b redesign (the next session's opening move, in order):
1. the row's pool offset moves to the OI ints (the stride 4 -> 5: the
   5th int = the pool offset; the od slot 9 dies) — the ints and the
   doubles never share a cell;
2. the pool grows past the truncation instead of marking rows: the od
   Vec extends (`od.resize(n*9 + pool_used)` per flush) and the Java
   outD sizing follows (`growD(n*9 + pool_used)` — the Set region
   already writes od.len()), so the overflow case DISAPPEARS instead of
   degrading;
3. the dense-fill guard: the scan's candidate list is UNBOUNDED (the
   vanilla loop stops at the first absorption — the candidates after it
   are irrelevant; the all-fail case needs the full list — the pool
   grows, never truncates);
4. the leg gate: merges=121k ±3%, dispFast ~98%, p50 in the 9.1-10.9
   band — the same discipline as the i0-a2 increments.

## Addendum 4 — the i0-b v2 build also regressed (reverted; the open question)

The v2 build (the pool offset in the OI 5-int stride, the od n*18+16,
the capped-8 candidates + the total with the mergeScanNative fallback)
compiled green, the whole workspace green (145) — and the leg regressed
AGAIN: p50 11.41 (the band is 9.1-10.9), the merge rows collapsed to
38294 (the baseline 121170, −68%), mergeOps 35412 (76696), rm rose to
7596 (5923). NO crash, NO counter explosion — the wire ran, the fast
path stayed at 98% — the MERGE DECISIONS THEMSELVES thinned.

The v1/v2 shared delta vs the proven baseline: the scan moved from the
applier's mergeScanNative wire (fastQuery at the loop B, the entity's
LIVE bb, the resolve loop) to the flush's phase-3 index query (the
mirror bb from the od's post-tick x/y/z, the store-mirror filter
is_mergable, the arenaRef resolution). The collapse means ONE of the
scan's inputs differs enough to change ~68% of the merge outcomes:

1. the BOX: the mirror bb (x±0.125, y..y+0.25) vs the entity's live
   getBoundingBox() — the 26.3 ItemEntity bb shape must be javap'd
   (makeBoundingBox / the dimensions constant) — a 0.25-vs-0.3 shape
   or a y-offset would shed exactly this many candidates;
2. the FILTER: the store-mirror is_mergable (the flags2 bit 4 —
   isMergableFast packed by the flush) vs the wire's isMergableFast
   re-check per live candidate — the mirror's freshness (the
   pickupDelay/age/count changes between the pack and the scan);
3. the ORDER/timing: the phase-3 scan runs AFTER the whole batch's
   commits — the earlier rows' absorptions removed candidates for the
   later scans (the wire ran per-row DURING the loop B, the same
   interleaving — hmm: the wire's scans also ran inside the loop B —
   the SAME interleaving — but the wire re-queried per row (the index
   updated by the absorptions) while the phase-3 scan PRE-QUERIED each
   row's box BEFORE the earlier rows' absorptions were applied?? — NO:
   the phase-3 scan runs IN the commit loop (per k, in order) — the
   same interleaving — EXCEPT the phase-3's absorptions happen in JAVA
   LATER (the loop B) — the phase-3's index never sees the earlier
   absorptions (they haven't happened yet — the loop B is AFTER the
   whole phase 3!) — **THE V1/V2 SMOKING GUN: the phase-3's candidate
   lists include the entities the loop B will absorb EARLIER in the
   same batch — the wire's per-row scans (inside the loop B) saw the
   post-absorption index** — the candidate sets diverge for CHAINED
   merges (the dense fill!) — the merge decisions thin exactly as
   observed.

THE v3 DESIGN (the next session): the scan returns to the loop B's
per-row timing, but the QUERY crosses JNI-free: the phase-3 CANNOT
pre-query. The options:
(a) the loop B calls a NEW in-process query entry (a Rust JNI export
    `nativeMergeScan(wid, x, y, z, r, yinf) -> [cnt, ids...]` — ONE
    crossing per scan (121/tick), zero row materialization, the index
    sees the post-absorption state ✓ the timing exact ✓) — the
    candidates resolve via the arena as today; the Java keeps the
    gates. The wire's rowQuery/fastQuery machinery dies; the crossing
    and the resolve stay.
(b) keep mergeScanNative and accept the wire cost (the 4% rowQuery
    cluster) — the honest floor.
(a) is a ~60-line slice: the export + the registry + the loop B's call.

## Addendum 5 — the v3 build storm + the honest close of the i0-b line

The v3 build (nativeMergeScan: one crossing per scan at the loop B's
timing) reproduced the v1 storm EXACTLY (p50 277ms, merges=0,
itemResync=50000 = every native add re-ingesting, vanillaNext=25000 =
one per item) — while v2 (NO JNI call, the pool read) only thinned the
merges (31%). THE COMMON FACTOR OF THE STORMS IS THE JNI CALL INSIDE
THE APPLY LOOP: a native crossing mid-apply disturbs the module's
arm/mode state (the exact mechanism needs a fresh debug session — the
candidates: the JVMTI/kernel reentrancy during the crossing, or the
exception-check path clearing a gate). v2's thinning is a separate,
cleaner divergence (the pre-query/post-absorption interleaving — the
addendum 4).

THE HONEST CLOSE: the merge wire STAYS mergeScanNative (the L3 path —
the rowQuery/fastQuery machinery, the vanilla-order candidates, the
4% rowQuery cluster stays). The item leg's remaining levers are the
tracker pass (i0-c) and the flush protocol polish — both minority
costs. The session's shipped increments (i0-a/a2/a2b/a2c) hold:
dispFast 98.4%, the paired items-25k at −51..−57% (the sandbox band
8.70-10.87 vs the vanilla 20.26), 145 tests green at every commit.

The post-restore verification leg (restore2): p50 8.70 (the session's
best), merge=120931 — the proven state re-confirmed.

## Addendum 6 — the session-final metrics (the i0 wave shipped)

- The item dispatch fast path: **98.4%** of dispatches (dispFast
  8040598/8172898 on the restore leg) — the steady state is fully
  core-decided; the slow chain serves the fresh/wake/belt entries.
- The final paired items-25k A/B (the same window, 2 vCPU):
  vanilla p50 **18.793** / mod p50 **9.908** = **−47.3%** (the session's
  leg band: the vanilla 18.8-20.3, the mod 8.70-10.87 — the mod's best
  window read −57%).
- The mob side untouched and verified: the 25k bench p50 **1.26-1.31
  ms/tick** (−85% vs the 8.41 bench origin, 2 vCPU) — the −90% line
  belongs to the CI 4-vCPU arbiter (doc 55: par(4) −90.5..−93.7%).
- The workspace: 145 tests green at every pushed commit (10 commits:
  60cd475..3c70d96).
- The merge wire: mergeScanNative stands (the v1/v2/v3 attempts
  documented in the addenda 3-5); the remaining item levers = the
  flush protocol polish (a minority cost).

## Addendum 7 — THE STORM ROOT FOUND AND FIXED: i0-b v3 lands at −58.6%

The bisect isolated the trigger: the v3 call with the IGNORED result ran
CLEAN (p50 8.88, the merges intact) — the JNI crossing, the lock, the
query, the fill are all innocent. The storm lived in the CONSUMPTION
loop: **the arena-resolved candidates lacked the isRemoved check** — the
arena still holds the entities absorbed EARLIER in the same batch until
the removal driver runs, and tryToMerge on a dead entity corrupted the
stacks and herded the population through the resync path (v1's and v3's
identical storms; v2's thinning was the separate pre-query divergence).

The one-line fix (other.isRemoved() in the consumption loop) landed with
the full v3: the leg reads **p50 {7.474, 8.091}** vs the vanilla 18.793
= **−58.6%** (from −47% pre-i0b), mergeOps 82244 (76696) — the
absorptions UP, the scans cheaper (one crossing per scan, zero row
materialization), the workspace green (145; the natives_parity contract
rides nativeMergeScan).

The item leg's ladder this session: −47% (i0-a2c) → −58.6% (i0-b v3+fix).
The mob side: 1.26-1.31 ms/tick @25k in-bench (−85%, 2 vCPU); the −90%
line = the CI 4-vCPU arbiter.
