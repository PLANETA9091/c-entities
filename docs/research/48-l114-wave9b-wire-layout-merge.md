# Doc 48 — L114 WAVE9b: THE WIRE-LAYOUT MERGE (the engine reads the MobStore)

## Verdict first

25k bench (seed 12648430, interleaved A/B, same binary):
**p50 2.36 vs 3.15 ms/tick (−25%, 5/5 runs)** — the merged resident engine
vs the wire arm (CE_FULL_PACK=1, the full-pack-per-round twin kept in the
same binary). 100k: **9.60 ms/tick** (was 12.65 in the wave8c window,
−24%). World parity: chasing **3745 == 3745** (25k) and **14008 == 14008**
(100k, 400 ticks) — the exact wave8 world counts. 21 suites green,
including the 400-tick resident-vs-full executor twin (THE merge gate:
whole-store dumps word-for-word over a chased/strolling/looking
population), the folded-vs-parked twin, the fused-vs-split twin and the
33 s natives_parity differential (the JNI-shaped wire path — untouched).

Session cumulative from the L112 start (8.41): **8.41 → 2.36 = −72%**.

## The lever: the pack was maintenance for a copy of the truth

The wave8 ledger named the end-state honestly: "the wire-layout merge
stays the end-state design". The pack (39% of the AI phase) existed to
copy the resident MobAI state into scratch words the engine could read;
the apply-side mirrors (19% of AI) existed to keep that copy honest
between rounds. The engine never wrote the scratch (`inb: &[i32]`) — it
is a PURE function of the words → (actions, cursor). So the copy (and
every store that maintained it) was removable if the engine could read
the resident fields instead.

## The change (goals.rs + mob.rs)

- `pub trait Src` — the engine's source view: header words (cursor/mf/
  tick_count/id/no_action/nav_state/mob_ts/game_time/seed/follow/
  counts/disabled), goal records (kind/prio/flags/cfg/cfg2/cfg3/
  running/state), players (pcount/pcoords/pflags/pvis), mob pos and the
  resume words (res/res_op). The engine (ai_floor_run + every helper
  below it) is generic over `W: Src` and monomorphizes per source — no
  vtable in the walk.
- `WireSrc` — the packed aiPack words: every accessor is the historical
  wire load, bit-for-bit the pre-merge behavior. `ai_floor_ext` (the JNI
  production entry, the fixtures, natives_parity) validates its guards
  and rides WireSrc — **production byte-for-byte unchanged**.
- `ResSrc` (swarm-sim) — the RESIDENT source over `&MobAI` +
  `&[PlayerSnap]`: kind/prio/cfg/cfg2/cfg3/running read the record
  fields; `state` replicates the pack's per-kind formula (LOOKAT derives
  the packed word on read); `flags` truncates to the low word and
  sign-extends back (the wire's `as i32 as i64` shape); `mob_ts` is the
  pack's static zero; `disabled` the pack's never-written zeros; `pvis`
  the 100% constant; `seed`/pos/resume words are the live fields. The
  round words (cursor/res/op) ride the struct — the scratch round-word
  writes die.
- `ai_floor_res` — the resident entry (the shared ext tail, no mob
  ledger — the wire-side diagnostic).
- `ai_tick`: the resident arm builds ResSrc per round and calls
  ai_floor_res — NO static/tick stamps, NO pack_head, NO round-word
  writes, NO seed scratch writes. `apply_actions::<const MIRROR: bool>`
  — the wire arm keeps every packed-word mirror (dead-code-eliminated in
  the resident instantiation); the resolver arm's `repack_goal_words` is
  wire-gated. The FULL twin arm (CE_FULL_PACK=1) keeps the exact
  pre-wave8 wire path: pack_into every round + the mirrors + the debug
  probe — it IS the merge's differential oracle.
- The retired Ctx::seed (a sign-extending formula nothing called) is
  gone; the trait's `seed` replicates the LCG-init shape (zero-extended
  low word) that ai_floor_run actually used.

## Why it is safe (the gates)

1. `resident_executor_matches_full_pack_executor` — the resident engine
   vs the wire engine, 400 ticks, whole-store dump per tick
   (positions/seeds/nav/goals/targets/approach/pend + the stroll stream
   + population). First run: PASS — the accessor bits are the wire bits
   by construction, the twin proves it every CI run.
2. `folded_executor_matches_parked_executor` — the fold contract over
   the resident engine, 400 ticks. PASS.
3. `fused_executor_matches_split_executor` (wave9a) — unchanged. PASS.
4. natives_parity (10 differential tests over the JNI-shaped wire) —
   the production path is untouched by construction and by test. PASS.
5. The world counts: 3745/14008 chasing — the exact wave8 numbers.

## What did NOT move (honest negatives)

- **The JNI production wire**: n_mob_ai_floor still packs per round on
  the Java side — the production floor now has the resident ENGINE
  available (`ai_floor_res`) but the Java bridge still speaks the round
  protocol. That is wave10 (the promised `n_mob_ai_tick`: Java packs
  once per mob-tick, the 32-round loop runs in Rust, resolutions ride
  the out actions).
- **The floor specialization** (the zombie table baked into
  straight-line per-goal code): the walk itself is now the top phase —
  the ledger reads (post-merge): ai ≈ 1.1 ms (floor ~80%, acts ~20%,
  pack ~0), travel ~0.9 ms fused inside it. The specialization is the
  next bench-visible lever.
- **The 32-round cap walk** (PH_CLEANUP/PH_UPDATE over 11 records per
  mob): generic by design; the specialization absorbs it.

## Tooling notes

- The goals.rs edit initially hit ENOSPC mid-write (the sandbox rootfs
  cycled full twice — the stale `/tmp/my-project` "duplicate" turned out
  to be a live PolarFS mount, NOT disk usage; the real consumers were
  the duplicate JDK copies). The truncated tail was restored from git
  and re-verified — the delivered file compiles clean and all suites
  pass from the repaired state.
- The module suite build recipe (doc 47) is unchanged: PAPER_JAR=
  ../paperboot263/versions/26.3/paper-26.3.jar + tools/jdk-25.0.5+8.

## Next (wave10 candidates)

1. **n_mob_ai_tick** — the JNI protocol: one call per mob-tick, the
   round loop in Rust, the resident slot as the wire (the production
   half of this merge).
2. **The floor specialization** — the zombie table as straight-line
   per-goal walk code behind a pack-level fuzz twin.
3. **The eval-walk branch profile** — the post-merge floor is ~80% of
   ai; perf-record the specialized walk for the branchless candidates.
