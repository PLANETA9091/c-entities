# 66 — B21: the kernel verdict walk (the walk-verdict precompute)

Date: 2026-11-22 (the -90%-everywhere push continues, session 29).
Inputs: the L190c gate (f044433, run 44939386733 PASS), the two explicit-
scenario profile flights on 5106ff3b (the post-L189 walls), Task 29
addendum 4's NEXT queue — (b2) the kernel-side roster.

## The honest standings at the slice point (L190c, each flight's own row)

mixed -97.5% (certified), spread -88.4% (55.18 -> 47.64 needed, -13.7%),
despawn -87.7% (11.30 -> 9.15), items -72.2% (the wall stands). The
profile-named Java share of the spread walk: tickListForEach ~7.0%,
itemTickFlush ~7.9%, ttlCachedPath ~5.5%, the Unsafe field-cache cluster
~9.4%, appendDispatch ~1.7%, quietFuseBump ~1.5%.

## The shape (what dies, what stays)

The walk's per-visit chain pays PER-ITEM RANDOM reads to re-derive
decisions the module already made at the last flush: the quiet arm reads
the DS_QUIET bitset + QUIET_REV (two random accesses into the 8 MB /
32 MB slot tables), the fast arm reads FAST_STAMP (the 8 MB stamp table).
In spread the walk order (TL order = insertion order) touches these
tables at effectively random offsets every tick.

B21 moves the DERIVATION to the kernel and keeps the DECISION half Java:

- ONE JNI crossing per armed walk (`nativeWalkPrep`): the walk hands the
  kernel the roster (the TL mirror's alive bitset + the id array), THIS
  wid's event belt, and the six scalars (curRev, curEpoch, stampRev,
  stampEpoch, quietRev, ovf); the kernel answers one verdict int per
  live roster slot: 0 = the exact Java chain, 1 = fast-native active,
  2 = fast-native inactive, 3 = quiet-owned. The walk consults wv[p] —
  a SEQUENTIAL read of the verdict array — instead of the per-item
  random reads.

The verdict semantics are the dispatch gate's STAMP half, mirrored:

- FAST (1/2): the entry's verdict field carries the SAME 18/20 bits the
  applier transcribes into FAST_STAMP (the i0-a2 decision — the kernel
  computed them itself); the validity gate `verdict_gen == flush_gen`
  mirrors the zeroed-stamp rule (aborts, resyncs and vanilla-ran rows
  keep the stale gen); the `(stampRev ^ stampEpoch) == (curRev ^
  curEpoch)` compare (masked to 31 bits) is the stamp's own field
  compare — revBefore is PRE-drain, epoch is the flush's local (the
  pre-bump MODE_EPOCH + 1), and the drain bumps WORLD_REV between them,
  so a drain tick stales every verdict exactly like it stales every
  stamp.
- QUIET (3): SET only via the belt — the applier's stampOk is Java-only
  (the SCAN_CLEAN belt and fullStackQuiet are Java state the kernel
  cannot derive); the mirror clears ride the commit's work-tail reset
  (the !B_TAIL_NONE F_NATIVE rows — the exact shape of the applier's L99
  persistent-quiet reset) plus the belt CLR events (l7ExternalMutation,
  the fuse bump's 256-tick belt). The walk's stale-but-boring re-stamp
  renews the belt SET with the refreshed rev.
- The DECISION half stays Java: a verdict-1/2 visit runs
  `itemDispatchItem(e, eid, true, v)` — the preVerdict overload skips
  ONLY the stamp read+compare; the L152 row-cohort check
  (`IB_IDS[ibN] == eid`) and the appendDispatch fallback are the
  untouched shape, so a drifted verdict can never mis-route a row.

## The exactness contract (what can and cannot drift)

- The prep REBUILDS the whole verdict array from the live roster every
  walk — the mirror compaction, the TL appends and the backfill cannot
  leave stale verdicts (the array lifecycle rides the TL_ROSTER regrow).
- The mid-walk invalidators are self-owned: removals (the roster's alive
  bit skips the slot), the vanilla bodies' own moves (l7ExternalMutation
  vetoes the slot via TL_MPOS and the belt CLR), and the fuse bump's own
  belt clear (the bump served the visit already). No mid-walk event can
  invalidate ANOTHER item's verdict — the walk is single-threaded and
  the vanilla bodies move only their own entity (items are not pushable).
- The belt is per-wid FIFO `(id<<1|kind)`; the events are one processing
  flush's transitions (rare by construction: the stamp transitions and
  the external mutations). Overflow drops events — the kernel resets
  EVERY quiet mirror and serves the tick all-zero (the exact chain), and
  the next flush re-stamps what the applier decides anew.
- The lost-drain rule: a failed prep does NOT reset the belt (a lost CLR
  must not lose its veto); the verdicts stay null and the exact chain
  serves the walk.
- CE_QUIET_FUSE=false (the bisect sweep mode) disables the consult: the
  sweep's bump runs OUTSIDE any walk and its belt events have no
  trustworthy wid. CE_WVERD=0 restores the per-visit reads wholesale.
- The multi-level shape: the belts are per-wid, the verdict state is
  per-wid (the store), the walk's wid resolves from the FIRST live
  roster entity's level (one memoized worldId call per walk). A wid >= 16
  (the map-path world) skips the belt and the stash — the verdicts go 0
  and the exact chain serves the walk.

## The expected effect and the honest math

Per quiet visit the walk drops TWO random reads (the bitset + QUIET_REV);
per fast visit ONE random read + compare (FAST_STAMP). The prep costs
the roster crossing (~1 MB at the 100k scale, ~100 us) + the kernel pass
(~0.15 ms) + the verdict write (~0.26 MB). Net at the spread scale:
~1.5-2 ms/tick off the ON leg (~3%), PLUS the itemDispatchWalk frame
skip for the verdict-served visits (~0.3-0.5 ms). spread 55.18 reads
~52.5-53.5 post-slice — HALF the 13.7% gap. The wire is the point: B22
(the kernel-owned fast rows — the BATCH_POS/BATCH_GEN/cohort/IB_* packing
leaves the thread for the verdict cohort) rides the SAME prep and carries
the remaining ~7-10%. items-100k's wall (the merge-volume physics) is a
separate arc (the honest merge volume + the BasicEntityList churn).

## Verification

- cargo: centities-core (51 tests) + centities-module (14 tests,
  natives_parity descriptor included) green on this box.
- javac: CeHooks.java compiles against the mojang-mapped paper-26.3.jar
  (JDK 25, --release 17).
- The flights: parity (the bit-identical judge), e2e, module, and the
  ab-paired gate (the medians judge) — CI arbitrates.

## The verdict (the CI arbitration, flight 45007660094)

ALL FOUR LEGS GREEN, PARITY BIT-IDENTICAL (12/12, zero ORDER/TLO
mismatches) — and the ON legs REGRESSED: spread ON 55.18 -> 67.37,
items ON 20.29 -> 26.92. The honest math after the flight: the verdict
coverage was ~19% of the visits (the wv counters), so the killed reads
(~0.5ms/tick) could not pay the prep pass (~5.5ms/tick at the 100k
scale — the despawn-10k leg measured the pure prep cost at 55ns/slot:
the per-slot 8-world Option loop + the branchy dirty binary search).

The b21fix stand (9bd6f4e9): the prep pass diet (the hoisted THIS-wid
world, the direct belt drain, the O(1) dirty byte map) + the consult
ships DORMANT (CE_WVERD=1 arms it; the Java belt appenders and the Rust
commit-mirror gate on the same env). The wire stays for B22 — the
kernel-owned fast rows (the BATCH/cohort/IB_* packing leaves the thread
for the verdict cohort) — where the prep cost pays for itself. The
parity discipline held: the live B21 was exact; the regression was pure
cost, caught by the honest cross-flight ON-leg comparison.
