# 64 — L177/L178: the fused quiet bump + the tracker's minimal bulk path

Date: 2026-11-18 (the -90%-everywhere storm, session 27).
Inputs: the L176 CI artifacts (8d9bd22, baseline run 44136474915) re-profiled
with the windowed leaf/line attribution (leaf_split.py + the new line_hot.py).

## The measure-window ranking (what was actually hot)

items-100k-mod (p50 11.58 ms): quietPass 18.65% (self), itemTickFlush 9.99%,
Unsafe.getObject 9.50%, tickListForEach 9.15%, AABB.intersects 6.31%,
newTrackerTick$ 4.92%, itemDispatch 4.72%.
mixed-150k-mod (p50 61.65): Unsafe.getObject 10.98%, ttlCachedPath 9.52%,
quietPass 6.81%, itemTickFlush 5.87%, itemDispatch 4.71%.
spread-100k-mod (p50 57.09): ttlCachedPath 10.72%, Unsafe.getObject 9.65%,
itemTickFlush 7.78%, itemDispatch 5.20%, quietPass 4.62%.

Line-level (line_hot.py): quietPass's cost = 41% sweep (the 7688 word walk
20.8% + the 7697 ARENA/instanceof/RM_BIT 15.2% + loads), 46% honest field
writes (7701 tickCount 25.3% + 7703 age 20.8%), 9.3% the UNCONDITIONAL
QUIET_FR[slot] load (7715) the open-tick check could have deferred.
ttlCachedPath's cost concentrates at 14509 (the rf[ob+1] read + the
ServerEntity cast — 82.7% of the method in spread) + updateDataBeforeSync
6.6% + the SIT steps isEmpty 5.6%.

## L177 — the fused quiet bump (CE_QUIET_FUSE, default on)

The walk (EntityTickList.forEach — javap-verified sole caller of the woven
body) visits every entity exactly once per tick per level: the SAME visit
vanilla's ItemEntity.tick pays. The quiet cohort's per-tick debt (tickCount/
totalEntityAge/age ++, the open-tick friction decay, the despawn + belt
exits) rides THAT visit inside dispatchQuietSkip's confirmed-quiet branch;
the ARENA bitset sweep (quietPass) never runs. Shape-exact vs the sweep:
same writes in the same order (friction first, despawn second, belt third);
the open-tick gate is pure arithmetic on the just-bumped tickCount + eid
(the true id — ARENA_ID[slot] without the load), and the QUIET_FR[q] load
is lazy (3-of-4 ticks skip the read the sweep paid every tick). The despawn
lands at the item's WALK position — vanilla's own position (the sweep fired
pre-flush); a slot the walk never visits (a non-entity-ticking chunk)
correctly misses the bump like vanilla (the sweep over-ticked those).
despawnTail mid-walk is vanilla-shaped (entities despawn inside the same
iteration; the moonrise set nulls the slot).
Counters: qui=..../fN(F|S) — fused bumps; CI parity arbitrates the exactness
(-Dcentities.quietFuse=false restores the sweep for the bisect).

## L178 — the tracker's minimal armed bulk path (TT_UBS)

The cached path's bulk rows (TT_OK, no SIT handler, mirror-owned counter)
run: identity read + shape read + the rf[ob+1] ServerEntity read + cast +
try frame + syncVelocity consume + updateDataBeforeSync + the mirror bump.
For a row whose updateDataBeforeSync() is PROVEN a no-op the whole mid-body
collapses to: syncVelocity + the bump. The proof is per CLASS at fill time
(ubsEmpty: the declaring-class walk — any override declares outside
Entity.class; Entity's own stub is the javap-proven EMPTY bc 19-23 shape
the trackerFastExit port already relies on), cached in UBS_PROOF and
stamped into the row shape (TT_UBS = 4L; the fill stamps se BEFORE TT_OK,
so a TT_OK row can never hold a null se — the full path's null-decline is
unreachable for the minimal path). Counters: ttlm=<mirror>/<ttlMin>.
Rows without the proof (players, overrides) take the unchanged full path.

## Local verification (2 vCPU, XMX 2500M, CI-shaped legs)

- parity items 20000: PASS bit-identical at T389/T639/final; the on leg's
  quietFuse == quietSkips exactly (8772767 = 8772767), belt exits 39959,
  despawns 0; ttlm=1/943737 of ttlc=1193080.
- parity spread 10000 (0.6 items / 0.2 stands / area 300): PASS
  bit-identical at all 3 snapshots; ttlMin 240564.
- module tests: 8 + itemcore 4 + natives-descriptor + patch suites green.
- local A/B items-50k pair: off 37.14 -> on 10.97 p50 (-70.5%); the mod side
  10.97 vs L174's 12.71 (-13.7% relative) — the sweep's removal on the same
  box (the off side swings with the runner; the on side is the stable read).

## What remains between here and -90%

The fused sweep + the minimal tracker path are the two biggest module-owned
families. The rest of the gap is the walk (9.15% self — the instanceof+arr
floor of the vanilla order), the honest merge volume (AABB.intersects +
invokeExact + the stack-walk filters ~12% — vanilla-determined), the
applier's per-row diffing, and the spawner/vec3Cached tails. CI arbitrates.
