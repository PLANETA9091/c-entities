# 42. The native pivot — Horizon study and the zero-Java entity architecture

Date: 2026-11-08. Status: DECISION + first slice landed (L110 = crate
`centities-native`, branch `l110-native`). Owner directive: rebuild the
acceleration architecture after the Horizon (github.com/horizon-rust/horizon)
study — entities ticked ENTIRELY in Rust, zero Java in the entity path,
then profile RUST functions and optimize the top of that profile.

## 1. What Horizon is

Minecraft 26.3 rewritten from scratch in Rust (MIT), verified against
vanilla with ~600 differential harnesses (per-tick AI-visible state
transcripts from a headless Java oracle vs the Rust probe — the same
discipline as our parity family, at engine scale). Key architecture facts
(all file refs = horizon-study clone):

- **Single-threaded, vanilla-ordered entity tick** (game/creatures.rs:1251,
  the comment at :1468 states the contract we already enforce: later mobs
  observe earlier mobs' tick results).
- **Plain AoS `Vec<Mob>`** (~3 KB per mob, insertion order) + `FxHashMap`
  id→slot cache (game/mob_index.rs:18) + per-tick flattened `Seen`
  snapshot (~40 fields) indexed by a **uniform 4.0-block XZ grid**
  (`SeenGrid`, seen_grid.rs:12, candidates() = guaranteed superset).
- **Attributes designed away**: flat `Copy` struct `Attrs` (mobs.rs:1017)
  copied from the immutable per-type `MobType` (Arc-shared); modifier math
  inlined per read (`Mob::movement_speed` mobs.rs:4100). No AttributeMap.
- **Block access = O(1) fact rows**: `BlockFacts` per block state (path
  type, collision top, flags), chunk hash-map + section palettes; A* uses
  precomputed `PathType` malus tables, not shape queries.
- **Side effects deferred**: goals push `MobEvent` (~110 variants) into one
  Vec, drained ONCE per tick by the host (handle_mob_events_reusable,
  creatures.rs:2322). The tick stays pure and diffable.
- **Built-in Rust profiler from day one**: thread-local span sampling
  (tick_profile.rs samples 1 tick in 17, per (level, mob kind) cost maps)
  + span traces; horizon-bench is just the report format.

## 2. What we already owned vs what the pivot adds

We had the hard parts and did not know their final shape:

| layer | status before the pivot |
|---|---|
| goal-selector floor | bit-exact vs Paper 1.21.8 (goals.rs, CI run #369) — but driven PER MOB PER ROUND through JNI (aiPack → nativeMobAiFloor → aiApply → resolve) |
| travel slice / push kernel / item tick | parity-proven in centities-core; items already ~100% native |
| world seam | resolved by JAVA inside the request loop (the L107 lesson: JNI calls invisible to JFR; fastMs/qMs timers are the only truth) |
| profiling | JFR = Java frames only; Rust-side cost invisible behind JNI |

The pivot (this branch): make the SAME floor the production executor with
the Horizon storage shape around it — the JNI round-trips collapse from
per-mob-per-round to ONE call per tick, and the world seam resolves in
Rust.

## 3. The new crate: `centities-native`

- **store.rs — MobStore**: persistent AoS `Vec<MobSlot>` (each slot = the
  parity-proven `MobAI` state + alive flag + grid cell) in vanilla
  insertion order; `HashMap` id→slot; tombstone compaction when free slots
  exceed `compact_at` (256). Tests: order stability, double-add guard.
- **store.rs — SeenGrid**: the Horizon 4.0-block XZ grid port; per-tick
  O(N) rebuild (their incremental re-bucket is the documented N1
  optimization); candidates() = superset with the >1024-cell overflow
  fallback to a full walk. Test: superset property incl. boundary cells.
- **world.rs — WorldRes**: the Resolver (swarm-sim's five-method protocol
  extended with two optional player-facing predicates — lookat/find-target
  — default false, so the sim is untouched semantically). Player targeting
  answers from the REAL pushed snapshot (O(P), the Horizon `players()`
  contract); `BlockFacts` is the seam where the N1 section-copy protocol
  plugs in (v0 = `OpenGround`, the named honest constructor).
- **exec.rs — NativeTick**: one call ticks the whole population: grid
  rebuild → player pack build → approach refresh → THE LOOP (ai_tick per
  mob, vanilla insertion order; the floor picks targets via
  ACT_REF_SET_PLAYER and apply_actions sets target+approach) → travel pass
  → compaction. The L111 bridge (Java side) will call this once per tick
  and becomes a COPY layer: push players + add/remove diffs, read back
  positions + events. No entity logic in Java.
- **profile.rs**: the phase ledger — ai/travel/grid/apply nanoseconds,
  sampled 1 tick in 17 (prime stride, the Horizon SAMPLE_EVERY trick),
  atomics, near-zero steady-state cost. `drain()` → report string.

## 4. First numbers (this sandbox, release build, single thread)

native_bench 25000 zombies + 1 player (the mobs-25k shape), 400 ticks,
warmup 50:

| metric | value |
|---|---|
| p50 / p90 / p99 | **8.79 / 9.48 / 12.28 ms/tick** |
| target acquisition | 3080 mobs chasing (find_target → ACT_REF_SET_PLAYER → chase confirmed) |
| phases (est) | ai 6.10, travel 2.12, grid 2.06, apply 0.11 ms/tick |

Reference points: vanilla Paper mobs-25k ≈ 494 ms/tick (2026-10-31 table,
doc 41); the current shipped module ≈ 240–260 ms/tick on that scenario
(the L109-era dAB ≈ +18–33% vs vanilla). The native executor is
**~28× under the module** and **~56× under vanilla** on the same shape —
the architecture headroom the pivot is built to harvest. The remaining
cost sits in ai (the floor walk) and travel/grid — exactly what
rust_perf_top.py now ranks per function.

9/9 tests green: store order/compaction, grid superset, executor
(no-players stroll, target acquisition + chase, bitwise determinism per
seed, remove/re-add exactness), profile ledger.

## 5. The profiler the user demanded

Two layers, both landed:

1. **Phase ledger** (in-crate, always on): the four phases above; sampled
   1-in-17; the bench prints it after every run.
2. **scripts/rust_perf_top.py + native-profile.yml CI job**: perf record
   `-e cycles:u` on the release bench (single-threaded, allocation-free
   steady loop) → parsed `perf report --no-children -s symbol,dso` → TOP-40
   RUST symbols by self-time; when the runner blocks perf_event_open the
   script falls back to the phase ledger automatically. The report goes to
   the job summary + artifact `native-profile-<run>`.

From this point every optimization claim can cite a RUST function line,
not a Java frame hidden behind JNI (the L107 discipline lesson, now
obsoleted — the executor is Rust end to end).

## 6. The N1..N4 phases (the rest of the pivot)

- **N1 — block facts for real**: the host pushes per-section copies (packed
  palettes) for the sections the managed population's boxes touch; a
  version stamp per section; `BlockFacts` impl reads palettes in Rust
  (Horizon's O(1) row contract). This replaces `OpenGround` — stroll
  targets get real safety checks, LoS gets the DDA walk.
- **N2 — the L111 bridge (Java copy layer)**: `CE_NATIVE_MOB=1` — the mob
  tick list collects add/remove diff + player snapshot + section
  subscriptions; one JNI call per tick (`nativeMobBatch`); read-back =
  positions/yaw/nav targets/events; vanilla predicates (see-through walls,
  despawn) ride the events. The Java body is NEVER entered for native
  mobs — the skip happens at the dispatch weave, before commonTick.
- **N3 — events outbox**: NativeEvent kinds (sound, drop, target set,
  despawn, damage) drained once per tick by the host applier — the
  Horizon handle_mob_events contract; this is also where vanilla side
  effects (zombie reinforcements, drops) get their exact semantics.
- **N4 — parity transcripts for the native path**: per-tick AI-visible
  state (position, motion, rotations, running goals, target, nav state)
  for every native mob, compared against the existing parity harness's
  A/B legs — the same bit-identical gates, now at the executor boundary.

## 7. Honesty ledger (the pivot's standing disclosure)

- EXACT (parity-proven machinery, unchanged): the goal-selector walk, the
  RNG stream semantics, the action set, the travel slice math, the
  push-kernel contract.
- v0 SCRIPTED (declared in code, changes WHERE mobs walk, never the cost
  structure): stroll target = splitmix draw + OpenGround facts (vanilla:
  10×7 draws with block-safety); LoS always true; path model = straight
  line (the open-ground routing proof); MeleeAttack approach = the target's
  live position.
- **L111b scripting addition**: `face()` yaw from the octant-folded
  minimax atan (atan(x) ≈ (π/4)x − x(|x|−1)(0.2447 + 0.0663|x|)),
  measured max reconstruction error 0.086° over all quadrants, quadrant
  edges exact. atan2's scale-invariance folds the normalization away —
  no sqrt, no divides, no libm atan2 per moving mob per tick. Chase
  timing shifts (3036 → 3027 chasing on the bench seed) — declared.
- The stroll stream is a separate splitmix (swarm-sim honesty note 1);
  the L111 bridge hands the real per-mob seeds.

## 8. Verdict discipline (unchanged)

No production claim until the N2 bridge runs under the existing gates:
parity bit-identical legs + baseline within-run dAB + native-profile
artifact. Until then `centities-native` is the kernel + bench + profiler
layer, additive only: nothing in the shipped config changes (main's
shipped stack stays L104+L104b+L106+L109).

## 9. L111a/L111b — the first math pass over the executor profile

The user's rule from the directive chain: parallelism is NOT
acceleration; acceleration is hard arithmetic. Two pure-math moves,
measured by interleaved A/B in ONE window (the sandbox absolute p50
drifts ~10% across minutes — 7.44/7.85/8.05/9.51 on near-identical
code; cross-window comparisons are noise. Only interleaved pairs and
within-run phase ledgers are evidence):

| phase (ms/tick, same window) | L110 | L111a/b | Δ |
|---|---|---|---|
| ai | 6.22 | 5.99 | −4% |
| travel | 2.31 | 1.91 | **−17%** |
| grid | 2.15 | 0.94 | **−56%** |
| p50 | 9.51 | 7.98 | **−16%** |

- **L111a (pure perf, no semantics)**: `pack_into` — the aiPack wire
  buffer is the mob's persistent scratch; the steady-state AI loop does
  ZERO heap allocations (was one Vec per round per mob). Byte-identical
  wire format, contract test pins it. `SeenGrid` — HashMap<(i64,i64),
  Vec<u32>> + SipHash + `div_euclid` → flat open-addressed table
  (zigzag u64 keys, murmur3 finalizer, linear probing), GENERATION
  stamps retire the previous tick with one integer add (no memset),
  counting-scatter into one contiguous vals Vec with prefix sums over
  the compact occupied list. `floor(x*0.25)` replaces the f64 divide —
  bit-identical (exact power-of-two scaling), roundss on x86-64.
  9/9 + 15/15 tests green; chasing count identical.
- **L111b (declared)**: the face() rewrite above. Plus a negative
  result recorded: removing pack_into's per-round zero-fill (words are
  all overwritten) measures ZERO in an interleaved A/B (7.96 vs
  7.99) — the sequential store pass is effectively free and the
  earlier regression reading was drift. The zero-fill stays.
- The ai phase is now 75% of the tick (5.99/7.98) — the goal-selector
  VM walk is the next and only remaining big fish; the N1 block-facts
  and N2 bridge work continues on top of it.

### 9.1 The no-root profile: PGO counters (L111d)

`perf` is unavailable in the sandbox (no root, no perf_event_open). The
working per-function instrument is PGO instrumentation + llvm-profdata
(rustup llvm-tools-preview, no root):

```sh
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release -p centities-native --example native_bench
target/release/examples/native_bench 25000 400 12648430
~/.rustup/.../bin/llvm-profdata merge -o /tmp/bench.profdata /tmp/pgo-data/*.profraw
~/.rustup/.../bin/llvm-profdata show --all-functions --counts /tmp/bench.profdata
```

Top of the profile (25k mobs x 450 ticks incl. warmup): the pack_into
zero-fill block 2.46G execs, `ai_floor` 201M, `can_use_inner` 51M,
`stop_goal` 16.6M, SeenGrid::rebuild 11.25M (= 25k x 450 — post-rewrite),
`exec::tick` 11.25M. Startup-only costs (MobStore index hash ~78k,
RawVec growth 25k) are already out of the steady state.

**LESSON (measured twice): block COUNT is not TIME.** The 2.46G block is
the buf zero-fill store loop (114 words x ~1.9 rounds ≈ 217/mob-tick —
exact match); deleting it measured ZERO in the interleaved A/B (§9
negative result) — a pure store loop on L1-hot lines is free. The
sub-phase Instant::now instrumentation attempt (guards per round) added
~190k clock calls/tick ≈ +7 ms/tick to the ai phase — REVERTED: at 25k
mobs the per-event clocking costs more than the phases it measures.
The honest per-function TIME split needs perf (root) or a sampler with
patched entry; until then: interleaved p50 pairs + phase ledger + PGO
counts are the evidence, in that order of trust.

### 9.2 L111e/f/g — the dead-work deletion, the one-pass tick, the INLINE floor

The user directive this round: "-100% on everything, do until done" (the
standing rule: parallelism is NOT acceleration; acceleration is hard
arithmetic — laziness IS: not computing what nobody reads is the purest
form of it).

**L111e — dead work deleted** (pure perf, zero semantics):
- `SeenGrid` rebuild is LAZY: epoch-stamped, rebuilt on the FIRST query.
  The v0 executor's AI never queries candidates (player scans walk the
  snapshot O(P)) — the per-tick rebuild was 0.94 ms/tick of nothing. The
  push kernel (N1+) pays for the build at its first query per tick.
- The per-tick `Vec<AiStat>` return allocated ~800KB EVERY tick (mmap +
  200 first-touch page faults); the buffer is now reused.
- Interleaved A/B: p50 7.24/7.37/7.32 -> 6.46/6.55/6.38 (**-12%**), grid
  phase 0.94 -> 0.00, chasing 3027 identical.

**L111f — one-pass tick**: AI + travel run per mob in ONE iteration (the
vanilla per-entity shape; the old two-pass shape touched every mob's
~11MB of cache lines twice per tick, the second pass cold, and violated
the "later mobs observe earlier mobs' POST-TRAVEL positions" contract).
Interleaved A/B: travel 2.0 -> 1.75-1.84, p50 -5%.

**L111g — the INLINE floor**: the parked protocol's per-request cost was
a FULL repack (114 words) + a floor re-entry (ctx rebuild + lock
rebuild); ~0.9 parks per mob-tick -> ~1.9 ai_floor entries. The new
`ai_floor_inline` resolves every request INSIDE the walk through the
`InlineRes` trait; the decision code is SHARED (one `run_mode`,
Mode::Parked | Mode::Inline) — the parked entry stays for the Java
bridge and the fixture. Live-state visibility (what the parked
protocol's apply-then-repack made visible to later rounds of the same
tick) is carried by two shadows + two hooks:
- `mf_live` (the MF_HAS_TARGET shadow): combat LOS winners set it, null
  results and the ZA/NEAREST canContinueToUse failures clear it —
  exactly the places the repack would have reflected them;
- `na_live` (the noActionTime shadow): every ACT_GOAL_START and every
  start body resets it (the parked repack made that visible to the
  STROLL canUse gate in the same tick);
- `ref_set_player` runs synchronously on the resolver (the later
  canuse_vanilla of the same walk reads the fresh target);
- `start_vanilla` draws through `&mut shared_seed` — the walk's LIVE
  SHARED_RANDOM position continues through the start bodies exactly
  where vanilla runs start() inline between the gates.

**THREE LATENT PARKED-PROTOCOL BUGS the fixture caught on the way** (all
fixed in the shared floor, both modes benefit):
1. **The cursor sentinel collision**: the resume token encoded 0 for the
   (TARGET, CLEANUP, gi=0) park — the HurtByTargetGoal
   canContinueToUse — indistinguishable from "fresh start": every
   re-entry re-parked the SAME request until the 32-round cap
   (FB_JAVA_ROUND_CAP). Bit-10 resume-valid added; the Java packer
   echoes the token verbatim, so this is transparent to CeHooks. The
   benches never hit it (their mobs are never hurt).
2. **The staging/action overlap**: `park_los` staged the candidate list
   at out[8..] — the SAME slots action 0 occupies. An LOS park that
   followed ANY action of the same walk (e.g. the CC resume's displaced
   stop) clobbered that action, and the executor's apply executed the
   staging bytes as opcodes (the fixture caught cands [len=1, idx=2]
   decoded as ACT_GOAL_START(goal 2)). The staging moved beyond the
   action region (`OUT_STAGE`, CeHooks AI_OUT mirrors the offset).
3. **The stale candidate-count read**: the parked LOS resolver read
   `n = out[8]` (now the action slot); reads `out[OUT_STAGE]`.

**The differential fixture** (`centities-native/tests/inline_parity.rs`):
3000 randomized single-tick scenarios + 200 chained 40-tick runs —
random players (0-3, mixed alive/spectator), random running sets
(flag-collision-free by construction), random state words, noActionTime
sweeping the 100 gate, nav states — asserts same status, same fallback
reason, same per-goal running/state, same targets, same seed, bit-equal
positions. On the FULL 400-tick bench: chasing 3740 = 3740 = 3740 (the
parked and inline executors produce bit-identical trajectories).

**Clean same-floor A/B** (both binaries carry the three fixes; the only
variable is Parked vs Inline):
| metric | new-PARKED | INLINE | delta |
|---|---|---|---|
| p50 ms/tick | 5.75 / 5.72 / 5.86 | 5.46 / 5.28 / 5.26 | **-7..-10%** |
| ai phase | 5.44 / 5.36 / 5.44 | 5.04 / 4.94 / 4.91 | **-8..-10%** |
| chasing | 3740 | 3740 | identical |

PGO counters after (25k x 450): `ai_floor_inline` 12.5M = **1.11
entries per mob-tick** (was ~1.9 re-entries), pack_into 2.13G block
execs (~189/mob-tick — one pack, still the top COUNT, measured free in
time: the L1 store stream), can_use_inner 179M (~16/mob-tick),
stop_goal 20.7M (~1.8 — the CC stops).

**The chasing shift 3027 -> 3740 is a CORRECTION, declared**: the old
bench trajectory carried the staging-overlap bug (garbage opcodes
applied) and the round-cap fallbacks suppressed acquisitions; the fixed
floor lets the correct acquisitions through. The L111b precedent
applies: the scripting shapes where mobs walk, never the cost
structure — and here the WHERE moved because the bugs moved.

**Session cumulative** (in-window comparisons, L110 baseline 8.41
ms/tick): 8.41 -> ~5.3 (-37%); the phase split now ai 4.9-5.0 (the
floor walk + the one pack), travel 1.72-1.74, grid 0.00, apply ~0.11.
The next big fish stays the same: the pack (114 words per mob-tick for
a walk that reads most of it) and the floor's gate walk itself.

### 9.3 L112a — the dead stores die (out-scratch reuse, the alloc-free scans)

The directive stands: -100% on every phase, do until done. Three
allocation/store sinks left in the steady-state AI loop, all eliminated
with zero semantic movement:

- **The OUT buffer is caller-owned** (exec.rs `out_scratch`): the old
  shape zeroed an 808-word stack array (`[0i32; OUT_LEN]` = 3.2KB) PER
  MOB PER TICK — 25k mobs x 808 words = **20M dead stores per tick**
  (~80MB of store traffic). The floor writes every slot the caller reads
  before the read (header words on every exit path, all 8 words of every
  pushed action; out[7] only under ST_FALLBACK where it is written), so
  the initialization was pure dead work. One persistent buffer serves
  the whole loop; the parity fixture now reuses its buffers ACROSS the
  40-tick chains, which pins the stale-word discipline itself into the
  contract (a stale read would poison the decisions and the fixture
  fires).
- **`scan_players` returns a stack array** `([usize; 32], usize)`: the
  scans run only under !MF_MANY_PLAYERS (>=31 players parks before the
  scan), so the bound is structural. The old `Vec<usize>` allocated once
  per Player-typed goal eval per mob-tick (three gates) — heap
  round-trips in the "zero steady-state allocation" executor.
- **`stop_goal` releases by flag bits**: lock[sel][bit] == gi implies
  bit in flags(gi) (every lock write site iterates the goal's own flag
  bits — the rebuild loops and lock_only), so the release walks 2-3
  trailing-zero bits instead of scanning all 16 lock slots per stop.
  Plus the inline adapter's `started` Vec -> a fixed `[usize; N_GOALS]`
  (one start per goal per walk — each canUse evaluates once).

**Interleaved A/B** (one window, alternating binaries, 25k x 400):
p50 5.18 / 5.16 / 5.17 -> **4.64 / 4.63 / 4.61** (**-10.4%**), wall
2180 -> 1945 ms, chasing **3740 = 3740 = 3740** (bit-identical
trajectories in all six runs). 86/86 tests green (the 3000-scenario
inline fixture included). Session cumulative against the L110 baseline:
8.41 -> 4.63 (**-45%**).

### 9.4 L112b — the running mask (the walk's set algebra)

The goal-selector walk kept the running set as `[[bool; 16]; 2]`: a
22-slot init pass (22 wire loads + 22 stack stores) per mob-tick and
11 test-and-branch goal probes per phase to discover the shapes. The
set is a bitmask now (`run_mask: [u32; 2]`):

- the init folds into register ORs (the wire loads remain — the pack
  wrote them);
- PH_CLEANUP skips entirely when `mask & run_low(count) == 0` (the idle
  mob shape — 11 probes -> ONE compare);
- PH_UPDATE skips when `mask & run_low(count) == run_low(count)` (the
  fully-locked shape — no canUse candidate can exist);
- PH_LOCK_REBUILD iterates ONLY the running goals via
  trailing_zeros (ascending order preserved — the lock-collide detection
  order is the walk order);
- the stop/start bookkeeping flips single bits.

Bits >= count are never set (init walks 0..count, starts happen below
count) and never read (every reader bounds by count or the walk's gi).

**Interleaved A/B** (one window): p50 4.70 / 4.67 / 4.75 ->
**4.56 / 4.60 / 4.54** (**-3.1%**), chasing 3740 identical in all six
runs, 86/86 tests green. Session cumulative: 8.41 -> 4.57 (**-45.7%**).
