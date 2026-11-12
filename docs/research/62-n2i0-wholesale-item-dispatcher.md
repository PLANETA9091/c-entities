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
