# 65 — L179: the TL mirror walk (the order-exact Entity[]-typed tick list)

Date: 2026-11-19 (the -90%-everywhere push continues, session 28).
Inputs: the L177/L178 flight artifacts (a55f718, baseline run 44151173012,
re-profiled with the restored windowed line attribution — line_hot.py).

## The measure-window ranking (what was actually hot)

The windowed line attribution (the deepest
`dev.planeta9091.centities.` frame owns the sample, the leaf splits into
callee buckets):

items-100k-mod (5660 samples, Server thread 4206):
tickListForEach 15.21% (line 9568 SELF 11.78% + line 9598 3.43%),
itemTickFlush 9.28%, quietFuseBump 6.57%, mirror 4.84% (its callee share =
AABB.intersects 3.04% + getBoundingBox 1.47% — the honest merge volume),
itemDispatch 4.36%, stackFast 3.55%.
spread-100k-mod (8810 samples): tickListForEach 16.16% (9568 SELF 10.22% +
9598 5.95% — the stands/mobs accept share), itemTickFlush 6.37%,
ttlCachedPath 5.09%, nativeMobBoundary 4.48%, standMirror 2.80%,
stackFast 2.78%.

Line 9568 = `if (o instanceof Entity e)` — the moonrise
IteratorSafeOrderedReferenceSet backing-array scan. Two costs per slot:

1. The array is NEVER compacted in our runs: moonrise's defrag fires only
   on a removal observed with iteratorCount == 0, and EVERY removal in the
   bench scenarios happens inside the walk (item merges, despawns) — the
   post-walk removal stream is empty, so the frag check never passes.
   items-100k-mod measure phase: ~62.9k live entities in a ~100k-slot
   array (the baseline.json entities column; the fill spawns 100k, the
   merge chains collapse ~37%).
2. Every non-null slot pays `instanceof` — a class-pointer read of the
   object HEADER: a second cache line per live entity (the ref itself is
   the first). ~63k header touches per tick per level.

## The moonrise semantics (javap-verified on 26.3)

IteratorSafeOrderedReferenceSet: `add` = putIfAbsent(e, listSize) — append
at the END, no hole reuse; `remove` = removeInt + listElements[idx] = null
+ firstInvalidIndex; `defrag` = shift-left in indexMap order (PRESERVES
order); the defrag fires only on a removal while iteratorCount == 0 AND
frag >= maxFragFactor (0.2). EntityTickList.add/remove = ensureActiveIsNotIterated
+ the set calls. The TL add/remove call sites: EntityLookup.entityStatusChange
→ worldCallback.onTickingStart/onTickingStop — the LevelCallback is
ServerLevel$EntityCallbacks (the lookup's own entityStartTicking/
entityEndTicking are EMPTY bodies for the server shape); onTickingStart
returns BEFORE entityTickList.add for marker stands when markers.tick=false
(bc 17-26), onTickingEnd has no marker gate.

## L179 — the TL mirror walk (CE_TL_MIRROR=0 restores the raw loop)

The module keeps its OWN Entity[]-typed copy of the tick list:

- probeTickStart / probeTickStop: prefixes on
  ServerLevel$EntityCallbacks.onTickingStart / onTickingEnd — the EXACT
  entityTickList.add / remove events. The mirror appends on start
  (TL_MPOS[id&AMASK] dedup matches the TL's putIfAbsent no-op), marks dead
  on stop (the alive bit clears, MPOS resets, DEAD++).
- The order is the TL order BY CONSTRUCTION — the same event stream, appends
  + the order-preserving compaction. No entity-id-order assumption: exact
  for ANY server shape, including chunk-ticking transitions (a re-entry
  re-appends at the end — the TL's own semantics) and mid-walk merges (the
  alive bit clears under the walk's feet, the slot is skipped — the raw
  walk sees the nulled TL slot).
- The walk: a word-skipping scan of the alive bitset (a dead slot costs one
  bit test in a hot 32-64KB structure; the raw walk pays a cold ref load +
  null check), live slots load a TYPED Entity[] ref with NO instanceof (no
  header deref). The per-entity body is the raw walk's body verbatim
  (collect → itemDispatch, mobRoute → nativeMobTick, else
  consumer.accept) — KEEP IN LOCKSTEP comment on both copies.
- Gates, every tick, raw stays the honest fallback:
  TL_MIRROR_OK (sticky) && TL_MIRROR_FLAG && backfilled &&
  TL_MIRROR_N >= TL_MIRROR_MIN (2048; the parity mobs leg 2500 arms) &&
  TL_MIRROR_N == indexMap.size() (the moonrise set's authoritative live
  membership — the probe-coverage proof: TL ⊆ mirror always (every TL
  member passed the probe on EntityLookup.addEntity), equality proves the
  sets are equal). Markers (probe fires, the TL add does not), a second
  ticking level, or any coverage hole make the equality fail → raw.
- The backfill: on the first gate-passing walk the pre-probe members are
  appended from the raw array in TL order; a count mismatch at backfill
  sticky-disables the mirror for the process lifetime.
- Compaction: dead >= len/4 triggers an order-preserving shift-left at
  walk END (never mid-iteration) — the moonrise defrag analog, with the
  MPOS re-stamps per moved entry.
- The PD-tick dual hash: at parity-dump ticks the raw array order is hashed
  exactly like the walked mirror order (the same golden-ratio mixer as the
  addendum-17 TLO). TLO-MIRROR-PROOF prints on equality; TLO-MISMATCH
  prints BOTH orders (first 48 ids) and sticky-disables the mirror — the
  TL parity stream made literal.
- The weave contract: the callbacks class loads with the FIRST ServerLevel
  (pre-arm) — the byte hook stashes, the verifier's deferred retransform
  applies the probes (rc + woven logged). Non-critical: a refusal keeps
  the raw walk (the count gate never passes without the probes). The
  counters ride the stats line: tlM=<mirror>/<raw>/<N>/<backfilled>/<ok>.

## Memory

TL_MIRROR Entity[1<<18] (2MB, doubles to the 1<<19 cap), ALIVE long[1<<12]
(32KB, doubles with the array), TL_MPOS int[1<<22] (16MB, the ARENA's
id-slot pattern), no fill needed (0 = not mirrored, index + 1 stored).

## Local verification (this box, XMX 2500M, CI-shaped legs)

- module tests: itemcore 4 + natives-descriptor 1 + patch suites
  (entity+lookup 1, patch_real 8) — ALL GREEN with the new probes.
- parity items 20000 (PARITY_SEED=4242, fill 200 / warmup 100 / measure
  600): PASS bit-identical at T389/T639/final; the mirror served 898 of
  the 1974 tick-phase walks (the raw walks = the fill phase below the
  2048 floor), TL_MIRROR_N=16498 == the TL live count every armed tick.
- parity spread 10000 (0.6 items / 0.2 stands / area 300 — the
  interleaved-population judge): PASS bit-identical at all 3 snapshots;
  tlM=878/2296, N=9901.
- local A/B items-50k pair (400 measure ticks, no JFR): off p50 26.46 ->
  on p50 7.23 (-72.7%; mean 53.47 -> 23.39). The mod side 7.23 vs the
  L176-era local 10.97 (-34% relative — the L177 fused quiet bump + L178
  tracker minimal path + L179 mirror walk stack). tlM=551/1423, N=25372
  == the samples' entity count exactly (the mirror == the TL membership,
  every tick of the measure window).

The probe weave landed on the FIRST dispatch attempt after the rework:
"TL mirror add/remove probes woven" in the arm log; the callbacks-class
attempt (the onTickingEnd same_frame@60 refusal) is preserved in the git
history as the reason the probes ride the primitives.

## What remains between here and -90%

The walk was the biggest module-owned family (11.78% self items / 10.22%
spread). The honest merge volume (AABB.intersects + getBoundingBox under
mirror ~4.8% items), ttlCachedPath 5.09% spread (the epoch-counter
redesign family), itemTickFlush 6.4-9.3%, and the spawner/vec3Cached tails
are the remaining families. CI arbitrates.
