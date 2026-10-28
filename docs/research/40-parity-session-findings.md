# Parity diagnostics — session findings (2026-10-28/29)

## Diagnostic arsenal (all committed, streaming into the CI job log)

| Tool | Input | Output |
|---|---|---|
| `PH t=<tick> h=<hex> n=<n> adds=<a>` | always on in parity | first diverging TICK per scenario |
| `PD t=<tick> id:m ...` | workflow input `ph_dump` (ranges, e.g. `155-165,175-190`) | per-entity state avalanche → the diff names the CULPRIT ENTITY |
| `PTRACE t=.. x=.. vx=.. tc=.. at=..` | workflow input `trace_id` | per-tick EXACT fields of ONE entity in both legs |
| `SR t=<tick> s=<hex>` / `LR t=<tick> a=<hex>` | always on in parity (fixed in da6dd04) | per-tick state of SHARED_RANDOM / Level.random — a diff names the FIRST TICK a stream desynced (=> different DRAW COUNT that tick) |

Analyzers: `scripts/ph_diff.py`, `scripts/ph_pair_diff.py`, `scripts/ptrace_diff.py`, `scripts/sr_diff.py` (in the local workspace, not the repo).

## Fixed this session

1. **Item move-gate off-by-one** (26b86af): `ServerLevel.tickNonPassenger`
   (26.3, javap) calls `commonTick()` (tickCount++) BEFORE `entity.tick()`
   — the `(tickCount + id) % 4` gate inside ItemEntity.tick reads the
   POST value, which equals the ingest value (collect +1). The
   `.wrapping_sub(1)` fired the gate one tick late for every grounded
   idle item (run 40754993404 item 29: vanilla moved at field-tc 15/19,
   module at 16/20). **items/mixed/spread divergences moved 155→159.**
2. **SR/LR probes** (da6dd04): Level.random in 26.3 is ALSO a
   ThreadUnsafeRandom (Paper patch) — the Xoroshiro bind silently failed
   forever; `value` is inherited (walk the hierarchy); SR emitted BEFORE
   the LR bind (an LR failure zeroed both streams silently).
3. **Shipped config = vanilla AI** (3d298d1): the L22a AI engine was
   forced on by the harness (CE_MOB_AI default 1) although the plugin
   ships with `centities.mobAi` default **0** and the engine is a
   measured net loss (e774d00: mobs-25k 193.2ms with vs 129.5 without).

## Open roots (in first-divergence order on the shipped config)

### R-A: L15 AI floor desyncs both RNG streams at t=143-145
The GoalSelector microkernel still engages at mobAi=0 (`gs=` counter
>0 in every shipped leg). Bisect 40773723794 (ai_floor=0, mobAi=1):
SR/LR clean until t=232 → the floor IS the t=143-145 source. Its
staged-request protocol consumes level.random with a different COUNT
than vanilla GoalSelector (an LCG endpoint only diverges on draw-count,
not order). **Next: ship aiFloor=0 (vanilla GoalSelector) like mobAi,
keep the floor opt-in until the doc-30 pre-resolved-predicate redesign
makes the staging provably draw-exact.**

### R-B: itemcore merge-scan candidate order (items t=159, mixed t=152)
SR/LR are IDENTICAL through t>1000 for items — the itemcore divergence
is pure deterministic math, NOT RNG. Run 40780021246 PD t=159: OFF
removes id=971 and keeps fresh id=1873; ON keeps 971 (its count grew —
it absorbed something) and loses 1873.

Bytecode-verified vanilla semantics (26.3):
- ActivationRange.checkIfActive: an ItemEntity is ACTIVE only when
  `(tickCount + id) % 4 == 0` (ifne 30 → the common path; ==0 returns
  true) — the activation gate COINCIDES with the move gate. The
  module's itemCheckActive gate `(field+1+id) % 4 == 0` is EQUIVALENT
  (the guard runs pre-commonTick, field+1 == the vanilla read) — NOT
  the bug.
- ItemEntity.tryToMerge(other): if `other.count < this.count` → this
  absorbs other; ELSE (tie included) → **other absorbs this — at equal
  counts the SCAN CANDIDATE survives and the TICKING item dies**.
  All parity items are STONE x1 → the first candidate always survives.

So the sole remaining variable is WHICH candidate is first in the
scan: vanilla scans `level.getEntitiesOfClass(ItemEntity, bb,
predicate)` in the moonrise ChunkEntitySlices order; the module's fast
merge scan (L37 vanilla-key order was proven for rowQuery) must return
the SAME first candidate. At t=159 the ON scan handed 971 a different
first candidate than OFF. **Next: dump the module's merge-scan
candidate list for 971's box at t=159 and diff it against the vanilla
getEntitiesOfClass order (the L37 ordSortVanilla must be applied to
the merge scan path too — verify it is reached at all for the
merge dispatch).**

### R-C: late non-RNG mobs divergence (t=183-320, AI knobs off)
With mob_ai=0 (40773720077): mobs PH diverges t=183 while SR/LR are
still clean (t=232) — a second, non-RNG bug. All-native-off (R1b):
t=320, exactly TWO entities (249, 1722) diverge in-place one tick —
candidates: L13 ticklist skip, tracker, or the ncolMh-broken push path.

### R-D: 26.3 migration — cramming audit bind broken (perf, not parity)
`lastErr=ncolMh: NoSuchMethodException LivingEntity.getRandom()` in
EVERY leg: 26.3 moved getRandom off LivingEntity → the push kernel's
cramming-audit MethodHandles fail to bind → the kernel declines →
vanilla push runs (correct but slow). Vanilla 26.3 cramming (javap):
`this.random.nextInt(4) == 0` gated on
`list.size() > maxEntityCramming - 1` — the audit uses the ENTITY's
random (SHARED), not level.random. Re-bind from net.minecraft.world.entity.Entity.

## Verified-clean subsystems
- itemcore vs RNG: items SR/LR identical through t>1000.
- push/collide knobs do NOT affect the t=143-145 stream desync.
- PH/PD integrity: re-summing the PD rows reproduces the PH hash (the
  analyzer must seed with phMix(tick) — the checker in ph_pair_diff.py
  omits the seed; cosmetic).
- vanilla-key order (ordSortVanilla) covers rowQuery; merge-scan path
  unverified (R-B).
