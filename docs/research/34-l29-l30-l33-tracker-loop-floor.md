# 34. L29/L30/L33 — the gate read floor, the tracker-loop floor, the tickEffects early-out

Date: 2026-10-27. Status: SHIPPED (commit 9306650). Basis: the flat
profiles of the 2ce981e run (mobs-25k -49.4%, mixed-150k -44.4%,
items-100k -79.6%, spread-100k -63.3%) — no 10%+ leaf, the remaining
cost is a thousand cuts. This round takes the three cuts with the best
mass/risk ratio that are pure-Java (no Rust surface changes).

## Where the time is (the 2ce981e profiles)

- items-100k-mod: the tracker family dominates — Unsafe.getObject 13.76
  (13.40 of it under trackerTickNoOp's seenBy read), trackerDispatch 8.86
  self, moonrise$getTrackedEntity 3.25, trackerTickNoOp 3.11,
  ReferenceOpenHashSet.isEmpty 2.25, sendChanges 1.12 (frame) — plus the
  tickListForEach walk 5.97. Every item pays the full moonrise$tick +
  sendChanges dispatch every tick even when NO player exists anywhere in
  the world.
- spread-100k-mod: trackerTickNoOp 2.61 + trackerDispatch 2.44 +
  moonrise$tick 1.54 + sendChanges 1.12 + newTrackerTick 2.17; the query
  family (EntityLookup.getEntities 3.54 + fastQuery 2.76 + AABB.intersects
  2.09 + ChunkEntitySlices 1.11 + rowQuery 1.08) is the other big bucket.
- mobs-25k-mod: setConverting$ 2.62 + setTicksFrozen$ 1.51 +
  setSharedFlag$ 1.43 + Unsafe 3.78 — the three data gates' 4-hop walks.
- mixed-150k-mod: the SynchedEntityData family (DataItem.getValue 3.63 +
  getItem 1.71) + the tracker family again.

## L29 — the gate read floor (precomputed accessor indices)

The three no-op gates (frozen/flags/converting) each pay a 4-hop walk per
call: `e.entityData -> itemsById -> items[idx] -> DataItem.value`, plus an
accessor instanceof and the accessor record's `id()` call. The accessor
ids are clinit-constant (defineId assigns once at the owner's static init
and the data array is indexed by exactly that id — javap-ground since L25),
so:

- `IDX_FROZEN`/`IDX_FLAGS` freeze at handle-init from the captured static
  accessors (the same Unsafe static-field capture the flag gate uses;
  identity preserved; instanceof'd before the record read);
- `entityDataValueIdx(e, idx)` is the lean walk — no acc instanceof, no
  record call; the `ed` instanceof stays (cheap, belt), the DataItem
  instanceof stays (the element type is NOT declared — this one is load
  bearing), the bounds check stays (a layout change degrades to the
  vanilla chain, never to a wrong read);
- `EDV_OK` (volatile, set once both indices froze) is the lean reader's
  single readiness gate; initFrozenHandles and initFlagHandles each can
  complete the freeze, whichever runs second flips it;
- the converting gate keeps its per-tracker accessor read (the accessor is
  an instance field) but caches the accessor IDENTITY in a one-slot cache
  (`CONV_ACC_CACHE`/`CONV_ACC_IDX`) — every tracker of a mob type holds
  the same static accessor, so the repeat call reads by the frozen index;
  a different accessor re-resolves, a non-accessor routes the proven walk.

All four call sites (setTicksFrozen$/getSharedFlag$/setSharedFlag$/
setConverting$) route the lean read first and the accessor walk on a null
(the miss then falls to the chain gate exactly as before). A lean miss on
the first call pays one extra walk once — negligible.

## L30 — the tracker-loop floor (ChunkMap.newTrackerTick)

The vanilla loop (javap, 26.3 — `newTrackerTick()` in full):

```
iteratingTrackerEntities = true;                    // bc 0-2
try {                                               // finally at 111-128
  raw = trackerEntities.getRawDataUnchecked();      // bc 6-14 (checkcast Entity[])
  size = trackerEntities.size();                    // bc 20-24
  for i in 0..size:
    te  = raw[i].moonrise$getTrackedEntity();       // bc 37-44 (one-field read)
    if (te == null) continue;                       // bc 46-51
    te.moonrise$tick(raw[i].moonrise$getChunkData().nearbyPlayers);  // bc 54-66
    if (!te.moonrise$hasPlayers()                   // bc 71-78
        || raw[i].moonrise$getChunkStatus().isOrAfter(ENTITY_TICKING)) {
      te.serverEntity.sendChanges();                // bc 97-102
    }
} finally { iteratingTrackerEntities = false; }
```

and `TrackedEntity.moonrise$tick(chunk)` (javap):

```
if (chunk == null) { clearPlayers(); return; }
players = chunk.getPlayers(VIEW_DISTANCE);
if (players == null) { clearPlayers(); return; }
oldLcu = lastChunkUpdate; update = chunk.getUpdateCount(); oldLtc = lastTrackedChunk;
lastChunkUpdate = update; lastTrackedChunk = chunk;      // the stamp writes
for p in players: updatePlayer(p);
if (oldLcu != update || oldLtc != chunk)                 // the removal pass
    for conn in new ArrayList<>(seenBy):
        if (!players.contains(conn.player)) removePlayer(conn.player);
```

and `moonrise$clearPlayers()`: `lastChunkUpdate = -1; lastTrackedChunk =
null; if (seenBy.isEmpty()) return; <removal loop>` — the empty-seenBy
early-out is VANILLA'S OWN (bc 12-24).

### The fused fast path

`hasPlayers()` is `!seenBy.isEmpty()` (javap). When NO player exists in
the picture — chunk null or its VIEW_DISTANCE list null/empty, seenBy
empty, ServerEntity.trackedPlayers empty, interp NO_OP, not hard-colliding,
no velocity sync — the vanilla pair collapses to:

- `moonrise$tick`: the stamp writes (value-invisible — the removal pass
  either runs over an empty seenBy or is skipped by the stamp compare;
  the ONLY other readers of lastChunkUpdate/lastTrackedChunk in the whole
  jar are the trackerTickNoOp gate's own probes, which reject on a stale
  value and route vanilla), the zero-iteration updatePlayer loop;
- `sendChanges`: the L24a-proven empty-tracking tail —
  `updateDataBeforeSync()` (bc 19-23) + `tickCount++` (bc 1106-1113), the
  changed-latches self-heal on the first real recipient.

So the guard fires BEFORE any state change and, when it passes, performs
exactly those two effects and continues the loop. Every probe is a pure
read (the L15b REORDER rule — running them before vanilla's moonrise$tick
order is free), and any structural doubt routes the exact vanilla pair.

### The loop mirror

The replacement body is the byte-exact loop above with ONE deviation:
`moonrise$getTrackedEntity()` is read via the Entity.trackedEntity field
offset when resolved (it is a one-field final read, javap bc 0-4) and via
the interface call otherwise. The `iteratingTrackerEntities` writes go
through a VarHandle/reflection (`setIterating`) — the field is a BOOLEAN
and a raw `Unsafe.putInt` would store 4 bytes into the 1-byte slot (the
SE_FAST hazard class); RefField.setInt is a raw putInt, so it must never
touch boolean fields (the same reasoning killed setInt for tickingEffects
in L33).

The mirror is ALWAYS the body — TTL_ON (armed after the ttlCalls liveness
proof, the standard verifier protocol) gates only the fast path. The
existing TrackedEntity.moonrise$tick guard and the ServerEntity.sendChanges
guard stay woven and serve the slow path exactly as before (the fast path
skips both call sites entirely; nothing double-charges).

Kill switch: `centities.trackerLoop` (default on). Counters: `ttl=`
calls/fuse/fuseChecks/armed in the hooks@ line.

## L33 — the tickEffects empty-map early-out

`LivingEntity.tickEffects()` (javap) with an EMPTY activeEffects map
still: allocates the keySet iterator (bc 21-26), writes
`tickingEffects=true` (bc 32-34) then `=false` in the finally (bc 157-159),
and polls the deferred queue (bc 162-176). Per mob per tick — 1.16-1.42%
of the mob/spread profiles. The guard (an EarlyReturnGuard2 prefix on
LivingEntity — the class already carries push + quiet-travel; the third
guard chains after whatever the mode lever kept, a refusal keeps the
base) replicates the empty case exactly:

- the loop runs zero iterations, and the only in-loop readers of
  tickingEffects are the MobEffectInstance.tickServer callbacks, which
  never fire — so skipping the true-write is unobservable;
- the finally semantics are reproduced: `tickingEffects=false` (typed
  VarHandle write — same 1-byte rule as L30) + the poll-run drain of
  deferredEffectUpdates, under the same flag value vanilla drains under;
- a drain runnable's exception propagates EXACTLY like vanilla's finally
  (the flag write already happened, already-run runnables stay run, the
  rest stay queued) — the guard does NOT catch past the drain;
- a non-empty map, a shape doubt or any handle failure routes the exact
  vanilla body.

The eff guard alone may apply when both base guards are refused
(CE_WEAVE_MODE bisect lever preserved). Kill switch:
`centities.tickEffects`. Counters: `eff=` calls/gate/armed.

## Weave inventory this commit

- ChunkMap (NEW class on the hook registry): replace_body →
  `CeHooks.newTrackerTick$` — the third replace_body on a tick-loop class
  (Level.getEntities, EntityTickList.forEach, now ChunkMap.newTrackerTick).
- LivingEntity: a THIRD EarlyReturnGuard2 in the same precompute chain.
- CeHooks: L29 idx freeze + lean walk + the L30 loop + the L33 gate —
  javac-verified against the real 26.3 jar under `--release 17` (the
  build.rs invocation; the binary `$`-names in source references do NOT
  resolve — use the `NearbyPlayers.TrackedChunk` dot syntax, javac's
  qualified-name resolution never finds `Outer$Inner` binary names in
  classpath jars).

## Expected effect

- items-100k: the tracker family was ~30%+ of tick samples — the fast
  path serves every item every tick (no players in the bench) —
  expected -8..-15% of the mod p50.
- mixed-150k/spread-100k: the same per-entity collapse for the untracked
  majority — expected -4..-8%.
- mobs-25k: smaller (the mob bench's trackers are mostly non-empty —
  mirror=78%); the L29 gate reads + the tickEffects early-out —
  expected -2..-4%.
- The gates (L29) cut across all scenarios: froz/conv/flag reads are
  15-20% leaner.

Watch the ab-paired run + the hooks@ counters: `ttl=` fuse should be
~100% of calls on items, `eff=` gate ~100% on mobs.
