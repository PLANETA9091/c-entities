# doc 53 — wave13: the shared cfg table (the per-mob copy dies)

Branch `main` @ wave13a. Session arc: **8.41 → 1.80 ms/tick p50 at 25k
(−78.6%)**; 100k × 400 = **7.08** (ledger was 7.55/7.82).

## The dead-end that wasn't

Doc 52 closed "GoalRec 20 B → 16 B" because cfg/cfg2/cfg3 carry f32 BITS
(the parity values themselves) and cannot narrow. True — and irrelevant:
the records never needed to be smaller, the cfg words needed to be
**gone from the resident struct**.

The executor contract already says the goal table is fixed at add_mob
(pack_fp polices it) — so cfg/cfg2/cfg3 are IMMUTABLE per mob AND
IDENTICAL for every mob of the table. Storing them per-mob was 132 B of
pure stream waste per mob: 25k × 132 B = **3.3 MB of L3/DRAM traffic
per tick for values that live in one cache line pair**.

## wave13a — the move

- `GoalRecR` (8 B: kind/prio/flags/running + state) replaces GoalRec
  (20 B) inside `MobAI.goals`. MobAI: 392 → 260 B (−34%).
- `ZOMBIE_CFG: [[i32; 3]; N_GOALS]` — the shared static, const-derived
  from the SAME `zombie_table()` (one source of truth; zero drift from
  the parity fixture). `zombie_table_res()` builds the compact records.
- The reads reroute: `ResSrc::{cfg,cfg2,cfg3}` and the static pack
  writer (`pack_into`, `pack_state_static`) index `ZOMBIE_CFG[flat]`.
  The wire words are untouched — byte-identical.
- `zombie_table()` itself (the full 20 B records) stays: the parity
  fixture, `static_fp` and the fp-sensitivity test keep reading it.

## Proof

- Census: **3745 == 3745** (25k × 400) and **14008 == 14008**
  (100k × 400) — the observable bit-stream is unchanged.
- Suites: **104 Rust tests green** on the local host (37 core, 10
  exactness, 8 itemcore, 19 native twins incl. the 400-tick whole-store
  dump twins, 22 sim, the rest aux). The module suite stays CI-side
  (mapped 26.3 jar + Java 25).

## Numbers (interleaved A/B, same host, worktree @ wave12b as OLD)

- 25k × 400, 5 rounds alternating: OLD 1.95 / 1.95 / 1.91 / 1.95 /
  1.92 vs NEW 1.79 / 1.80 / 1.82 / 1.80 / 1.79 — **1.95 → 1.80 p50
  (−7.7%, 5/5)**.
- 100k × 400, 3 rounds: OLD 8.32 / 8.28 / 8.22 vs NEW 7.58 / 7.59 /
  7.58 — **−8.5%, 3/3** (the DRAM lever compounds with scale, as
  wave11c predicted); 100k × 400 fresh run: **7.08**.
- p90 at 25k: 2.44 → 2.07 (the tail is cache-miss pain; the shrink
  feeds it directly).

## The face() probe (measured, closed)

A temporary counter: **3429 face() calls per tick** (13.7% of mobs are
movers). The glibc atan2 clone's ceiling is therefore ~0.03 ms/tick
(−1.5%) — below the risk bar for hand-porting glibc's IFUNC-selected
FMA variant bit-exactly. Closed without code.

## Where the honest remainder sits (post-wave13)

- AI ~0.78 ms (floor ~0.5 + acts ~0.28): the RNG-serial eval stream is
  the semantics; the straight-line shapes own 100% of the bench
  census (even_gen 0 / odd_gen 0).
- travel ~0.30 ms: move_tick's chain + face() at 13.7% coverage; the
  two divisions + sqrt are IEEE-exact primitives (bit-locked), the
  atan2 prize measured small (above).
- skeleton ~0.12 ms: the fused loop + the ledger instrumentation
  (fences ~2 ns/mob when the probe is off).
- cache interference ~0.2 ms: the fused single pass streams 25k ×
  260 B = 6.5 MB through one core's L3 — was 9.8 MB pre-wave13.

The next honest magnitude still needs a relaxed constraint, but the
floor moved: **~1.6-1.7 → ~1.5-1.6 ms at 25k** within the standing
constraints.
