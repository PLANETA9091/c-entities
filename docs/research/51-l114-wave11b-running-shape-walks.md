# doc 51 — L114 wave11b: the running-shape walks (slices two + three)

Branch `main` @ `1b81b8f`. Session total: **8.41 → 2.31 ms/tick p50 (−72.5%)**.

## The shape census (the TEMP counters, the wave11a blind spot closed)

The wave11a commit shipped z0 with the claim "everything outside the shape
rides the generic engine" — but never counted the shapes. The thread-local
shape counters (the bench prints them; zero atomics — the first version
paid ~0.3 ms/tick in LOCK XADDs, the wave2 lesson again):

```
shapes | even_z0 2422583 (21.5%) | even_z1 3214917 (28.6%) | even_gen 0
       | odd_skip 3598048 (32.0%) | odd_z2 2014452 (17.9%) | odd_gen 0
```

**46.5% of zombie mob-ticks still rode the generic floor** (the chasers,
the strollers, the RLA-active, the lookat-active). The fine probe's
cleanup 30.6% + lockreb 28.7% + update 40.6% split (doc 49/50) was
measured over exactly those calls.

## What shipped

- **ai_floor_z1** — the even running-shape walk, straight-line over the
  table's static record positions: the cleanup stop bodies (the disabled
  gate, the cc fold for the vanilla-cc kinds, the ported cc for
  stroll/lookat/lookaround, the ACT_TARGET_CLEAR mf-rebase tail), the
  FB_LOCK_COLLIDE lock rebuild, the update under locks — WITH the
  displaced-stop machinery live: the zombie table's single-flag records
  make a start displace a RUNNING holder exactly when the candidate's
  priority is lower (SPEAR(2)/MTV(6) over RLA(8)/STROLL(7)) —
  ACT_GOAL_STOP_D, the vanilla map.put. The lock slots are provably
  running-or-empty (single-flag records), so LOOKAROUND's map-only lock
  walk stays the exact z0 shape. The mf word is a mutable local (the
  generic ctx.mf_ov tail). The stop_goal lock-clear tail is omitted: on a
  fresh tick the lock tables are untouched-empty at both cleanup points.
- **ai_floor_z2** — the odd tickRunningGoals(false) body straight-line:
  LOOKAROUND's ported tick + the melee virtual, in record order per
  selector, then the done words; NO RNG draws (the seed round-trips
  verbatim), never parks (one round). The caller's skip ate the empty
  shape; z2 owns the et_running shape.

The generic floor now serves **zero** even zombie mob-ticks and **zero**
odd et mob-ticks in the bench (only the park resumes (cursor != 0) and
non-zombie tables remain generic).

## The probes (what the tail actually costs)

- apply n==0 **5 ns**, one ACT_LOOK_TICK_AROUND **13 ns** — the action
  round-trip is cheap; no reason to bypass the contract.
- the fixed tail (fold_ctx + out fill + seed handoff) **14 ns/mob-tick**.
- move_tick **33 ns/call** all classes; the face() atan2 chain **15 ns**
  (glibc f64 atan2 — not the monster, the streaming is).
- the rdtsc section fences split ai_tick: **A entry+streaming ~45 ns,
  B pre-floor ~35 ns, C floor ~75 ns, D seed+apply ~25 ns** (the
  Instant::now fences cost ~100 ns each — the rdtsc version is the
  usable one; the wave2 meter's-shadow lesson, third verse).
- CE_NO_AI / CE_NO_ACTS / CE_NO_TRAVEL probes: executor loop ~0.34 ms,
  travel ~0.53 ms, acts ~0.14 ms, floor+preamble ~0.8 ms of the 2.31.

## Dead ends (measured, closed)

- **The atomic shape counters** — LOCK XADD per mob-tick on shared lines:
  +0.3 ms/tick on BOTH arms; thread-local Cells are free.
- **Instant::now section fences** — the clock call IS the profile
  (~100 ns each on this sandbox); rdtsc or nothing.
- **The 8-mob pipelined eval stream** (doc 50, carried) — the AoS gather
  eats the latency win.

## Numbers

- 25k A/B (interleaved): **2.31 vs 2.70 p50 (−14.5% arm-to-arm)**,
  chasing 3745 == 3745.
- 100k x 400: 9.42 chasing 14008==14008 (the wave10 marker).
- Suites: 17 native + 37 core + 21 sim green; the zstrait twin drives
  z1+z2 (400-tick whole-store dumps) first run.
- Session: **−72.5% from 8.41**.

## Next levers (the honest ceiling math)

The floor (C ~75 ns) is at the bare-eval minimum (the serial LCG chain +
the gate branches; the probe's floor is ~53 ns). The remaining fat:

1. **The population streaming (A ~45 ns/mob-tick)** — the 568–600 B
   MobAI/MobSlot stride: goals[11] × ~40 B GoalRec = ~73% of the struct.
   The RESIDENT layout is free to shrink (the JNI wire is derived by
   pack_into, byte-identical regardless) — a ~20 B GoalRec saves ~220 B
   per mob of L3 traffic per pass. The 100k lever; the 25k is
   L3-resident.
2. **The pre-floor tail (B ~35 ns)** — the any_running 11-record scan
   per even tick can be a maintained bit (the et_running pattern);
   the fold_ctx build could ride into the walk.
3. **Travel (~0.53 ms)** — the movers' face/get_input_vector math
   (bit-exact atan2 ~15 ns) + the streaming; the friction chain is
   already folded (wave6).
4. The executor loop (~0.34 ms) — the store iteration + stats + grid.

The −90% goal (0.84 ms/tick from 8.41) sits below the measured floor of
the current architecture (the eval stream alone is ~0.3 ms at 25k, the
streaming ~0.4 ms, travel ~0.5 ms): the next honest steps are the struct
shrink (1) + the B-tail bits (2), then re-measure whether the plateau
holds.

## Environment notes

- The sandbox recycled again: rustup rebuilt (1.99.1), the paper jar
  re-downloaded (26.3 build 248, sha256 pinned in the fetch). The module
  suite needs Java 25 (paper 26.3 refuses 21) — the Temurin 25 tarball is
  the recipe; natives_parity pending the mapped-jar boot.
