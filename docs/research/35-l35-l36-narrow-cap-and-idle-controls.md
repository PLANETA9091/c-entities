# 35/36. L35 narrow-shape row cap + L36 idle AI-control floors

Date: 2026-10-27. Status: SHIPPED (L35 @ 2ae5720, L36 @ 30575ed).
Basis: the 4f49049 profile (mobs-25k-mod 184.8ms, -50.7%) — the top-15
is flat (no leaf over 5%), the mirror family (AABB.intersects 5.8% +
ChunkEntitySlices 1.35 + getId 1.45) and the per-mob AI-control frames
are the largest remaining buckets.

## L35 — narrow-shape row cap

The flat ROW_TOTAL_CAP=512 sent every query whose covered rows totaled
>512 candidates into the native/mirror paths. For NARROW shapes that
was the wrong trade: the L4c packed-x-window scan (row entries carry
(id, xLo, xHi) — a conservative 1/1024-block x-interval) skips 75-96%
of the row at ~3-4ns per entry WITHOUT dereferencing the Entity, so
the expensive per-candidate Java path (ARENA deref + home re-check +
live-box intersects + predicate) applies only to the x-overlap
fraction. A mob collision query (~0.6-1.9 blocks) or an item merge
scan (~1-2.5 blocks) in a 400-candidate row now pays ~400 × 3.5ns +
~40 × 50ns instead of a ~13-30us JNI+drain+publish toll.

- Cap: ROW_TOTAL_CAP_NARROW = 8192 for boxes with X and Z spans both
  <= 8.0; wide shapes (targeting/sensor scans 7-35 blocks) keep 512 —
  their window pass fraction is near 100%, so there the scan cost is
  per-CANDIDATE and the native Rust filter wins (the L4c spread-100k
  lesson, unchanged).
- The bound is a belt, not the real ceiling: valid rows are each
  <= ROW_CAP (510) entries and a narrow box covers <= 4x4 = 16 rows
  (max 8160). Dense sentinel rows (>510/cell) still refuse.
- The EMA still prices every call-shape bucket and flips to the mirror
  when the row scan loses — the change can only move work INTO the
  already-proven exact row path (home re-check + live-box intersects +
  predicate unchanged), never alter an answer.
- Counter: `row=` tail gains `/n<rowNarrow>` (narrow-shape serves).

## L36 — idle AI-control floors

A standing mob's serverAiStep tail calls three control objects every
tick. javap (26.3) proves each body collapses to same-value writes for
a settled, pathless mob, so a prefix guard can skip the whole frame:

- PathNavigation.tick: the body is `tick++; if (hasDelayedRecomputation)
  recomputePath(); if (!isDone()) {...}` — with path==null &&
  !hasDelayedRecomputation the only effect is the private tick counter
  (bc 0-19; the whole rest sits under bc 21-25's `!isDone()`).
- MoveControl.tick: the WAIT branch (bc 579-584) is exactly
  `mob.setZza(0.0F)` — a plain putfield; at operation==WAIT && zza==0
  it is a same-value store. (Operation is PROTECTED — the capture rides
  Enum.valueOf, same as the L22 init.)
- LookControl.tick: setXRot(0F) is same-value at xRot==0 (and
  resetXRotOnTick() needs no read — both outcomes of the override are
  invisible under xRot==0); the cooldown branch is skipped at
  lookAtCooldown<=0; the else branch writes yHeadRot =
  rotateTowards(yHeadRot, yBodyRot, 10) whose from==to fixed point is
  exact (Mth.rotateTowards returns from + 0); clampHeadRotationToBody
  is gated on !navigation.isDone(). Guard: xRot==0 && cooldown<=0 &&
  yHeadRot==yBodyRot && nav.isDone() — the isDone() call stays VIRTUAL
  so any GroundPathNavigation override remains authoritative.

Weave: three independent op-4 EARLY_RETURN_GUARD prefixes (no-arg
instance methods — the tickEffects op choice; op-5 refuses no-arg
methods). Each has its own kill switch (centities.navIdle/moveIdle/
lookIdle), its own handle-proof (initNav/Move/LookHandles + the
*_HANDLES_OK flag read by prove_handles) and the stash/retry deferred
weave (the classes load with the first mobs, the same wave as Sensing —
the verifier block re-runs precompute_idle_controls and retransforms
only the classes whose weave landed).

RefField gained getBool (Unsafe.getByte — the L33 1-byte boolean rule;
a raw getInt on a 1-byte slot straddles the adjacent field) and
getFloat. Counters: `idle=` nav/move/look calls + the three hit
counters.

Behavior parity of BOTH levers is asserted by the parity A/B
(bit-exact snapshots across off/on legs) — any drift in RNG draw
counts (none of these guards consumes RNG), goal decisions or
movement diverges the snapshot sha256s and fails the gate.
