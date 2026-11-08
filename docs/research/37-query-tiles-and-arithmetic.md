# 37. Query arithmetic wave: 4x4 tile sub-partition, packed order word, no-probe merge

Session goal (user directive): «продолжи ещё жожще оптимизировать с аритметиками
и вообщем всем» — push the arithmetic everywhere, measured.

## Phase isolation first (iso_bench example)

Split the query into walk / walk+order / full on the two canonical workloads
(100k queries each, release, this host):

| phase    | dense-pile | spread |
|----------|-----------:|-------:|
| walk     |    28.6 ns |  55 ns |
| walk+ord |    33.9 ns |  50 ns |
| full     |  1208.6 ns |  56 ns |

Verdict: the dense query cost is 97% SCAN (the walk + version check are
noise). The scan visits ~789 candidates per query (ret_avg of the walk probe)
to emit 21 exact survivors — 2.7% selectivity — at ~1.5 ns/candidate. The
lever is therefore NOT micro-uops: it is scanning fewer candidates.

## Wave 1 (committed 3c92bce)

* `simd.rs`: the AVX2 scan loop is split — `n_blocks-1` full blocks run
  unmasked, the tail block is handled once after the loop. The per-block
  `if blk+1==n_blocks` branch is gone.
* `lib.rs`: `cell_ov`+`cell_sv` packed into one `cell_vers: u64`
  (low32 = current order version, high32 = version sorted to). The per-cell
  order check is ONE load + one compare instead of two loads. Bump sites use
  `(*v & HIGH) | ((*v+1) & LOW)` — carry-safe, wrapping like before.

Effect: within noise on the scan-bound dense workload (expected — the scan
is memory-latency bound, not uop bound); the order check halves its memory
traffic, which matters for the multi-cell walk.

## Wave 2 — the main event: 4x4 XZ tile sub-partition (doc 37)

**Design.** Dense cells (>= QUAD_MIN=16 members) carry, next to the parent
blocked-SoA store, a 4x4 XZ split: tile q holds the members whose stored-f32
box CENTER falls in the 4x4-block band (tx = floor(cx/4), tz = floor(cz/4),
q = tx | tz<<2 — careful: a bit-flag encoding collapses bands 2/3, the band
index must be the LINEAR floor division; the f32 sum x0+x1 is twice the
center, so band = floor(sum/8), clamped to 3 for the 15.99…→16.0 rounding
edge). Each tile store is a self-contained blocked-SoA run; the partition is
STABLE, so each tile keeps the parent's (sy,seq) vanilla order.

**Skip proof (no false negatives).** A member of tile q has f64 center c in
[xb*4, xb*4+4) and stored half-span <= mh[q]. If the query box expanded by
mh[q]+EPSF (1e-2, the doc-21 slack, 1000x above ULP(24)) is disjoint from
the tile range, every member's exact f64 test fails on that axis: skip the
tile. mh[q] is derived in f64 from the same f32s the exact check promotes —
no extra rounding beyond doc 21's covered envelope.

**Order parity.** One emitting tile → its survivors are already
(sy,seq)-ascending: emit directly. Several emitting tiles → k-way merge by
key. First implementation merged through the existing keyed sort
(`order_key_of` → idmap probes): measured 283 ns/query — the whole win of
the 2x2 prototype eaten. Fix: each tile carries a per-candidate `keys`
array (the kb stamp, built in `build_quad` for free), `scan_cells_keys`
pushes the key next to every id, and the merge is a linear-min k-way over
the emitting segments in `merge_segs` scratch — no hashing, ~tens of ns.

**Publisher untouched.** rowQuery/cell_scan/cell_scan_packed read the parent
store in its (sy,seq) physical order — the tiles are separate copies, the
vanilla-order contract of the row path is untouched.

## Measured (canonical 64-pile + spread, 100k queries)

| metric                     | before | after  | ratio |
|----------------------------|-------:|-------:|------:|
| scanned candidates/query   | 789    | 164    | 4.8x  |
| dense full query           | 1230 ns| 775-1005 ns | ~1.4-1.6x |
| spread full query          | 55 ns  | 55 ns  | 1.0x (untouched) |

The residual dense cost is now the constant per-query floor (walk 28 +
push/merge/setup ~300) + 164 cands of scan — the next levers are the
per-cell overheads, not the scan.

## Correctness gates

* `quadrant_dense_cells_no_miss_and_order` (new): 500-mob dense cluster
  (all cells >= QUAD_MIN), 4000-round mutation storm, every 5th round a
  verification query: (1) no-miss vs brute force, (2) the exact-hit
  subsequence equals the vanilla key reference — the full
  (chunkZ, chunkX, sy, seq) order, re-appended stamps modeled, (3) scalar
  vs AVX2 identical sequences on the tile path, (4) invariants.
* Full core suite: 44 tests green; module suite: 24 tests green
  (incl. natives_parity descriptor drift — the test's hardcoded list was
  synced to the sy/seq batch arrays of doc 36b).

## Bugs caught by the harness (worth remembering)

1. Tile bits scrambled: x/z bits interleaved as (bit0=x-fine, bit1=z-fine,
   bit2=x-coarse...) while the query read (qb&3) as x — mixing axes.
2. Band encoding: (>=4 | >=8<<1) flags collapse [8,12) and [12,16) to the
   same code 3 — the linear floor(sum/8) encoding is required.
3. The test's own reference order initially ignored vanilla re-append
   (a re-upserted entity takes a NEW stamp and moves to the END) and used
   `swap_remove` (reorders the reference) — fixed to model vanilla.
4. The query contract returns a margin-expanded SUPERSET — the order
   assert must compare the exact-hit subsequence, not the raw output.

## Toolchain notes (session recovery)

/tmp was wiped between sessions: cargo/JDK/liveries had to be reinstalled.
The module build needs PAPER_JAR=<paper-26.3.jar> + a `libraries/` dir next
to it (paperclip -Dpaperclip.extractonly=true provides both; JDK 25 for
26.3). Both now live under /home/z/my-project/work/ (survives recycles).
Bash output eats `[ho`/`[m`-looking fragments (ANSI-sanitizer artifact):
verify "corrupted" code with `od -c` before editing — this session's
«broken cell_ovome» was a display artifact, the file was intact.
