# doc 63 — L175: the stand-data port wakes up (the gate read an unassigned static)

Branch `main` @ 97a5d02. The 25-addendum listed the spread mod profile's
next target: "the tracker cached-path gate reads — SynchedEntityData.
getItem/getValue 14.9% + isRemoved/getType/getHealth ~3.9%". The artifact
re-analysis named the caller and the caller's fix turned out to be a
one-line bug verdict.

## The attribution (CI 567b6ca spread-100k-mod rec.jfr, 8812 samples)

leaf_caller_attribution.py over the CI artifact:

| leaf | % of profile | top caller chain |
|---|---:|---|
| SynchedEntityData.getItem | 8.39 | **94.6%** = `ArmorStand.isMarker()` ← `CeHooks.standMirror` ← `itemDispatch` |
| DataItem.getValue | 6.48 | 79.5% isMarker, 6.3% `Entity.isNoGravity()`, 4.7% `LivingEntity.getHealth()` — all under standMirror |
| LivingEntity.getHealth | 0.93 | standMirror (isAlive) |

Any-frame family total: **standMirror = 15.30% of the whole mod profile**
(getItem 7.93 + getValue 5.87 + getHealth 0.84 + the frames themselves).
The largest single family in the profile — bigger than tickListForEach
(5.46%). 20k stands × 3 SynchedEntityData chains per tick.

## The verdict: the L165 port shipped DEAD

standMirror had a direct-offset port (STAND_PORT, L165) that read the
SAME value slots through Unsafe (`entityData -> itemsById[] ->
DataItem.value`) — javap-verified byte-exact (`SynchedEntityData.get(acc)`
= `itemsById[acc.id()].value`; `isMarker` = `(byte & 0x10) != 0`;
`isNoGravity` = the Boolean). It never ran: the gate checked
`O_SDI_VALUE >= 0`, a static that had NO assignment anywhere in the file
— the value offset was resolved into the older L19 name `O_DI_VALUE` on
the very same line it was needed. `O_SDI_VALUE` stayed -1 forever, the
port condition was false, every stand rode the vanilla chains, and the
`standPortHit/standPortMiss` counters (0 reads) watched silently. The
fix: the gate + the reads use `O_DI_VALUE`.

## L175b: the isAlive port

The same profile showed the third chain under standMirror: `!as.isAlive()`
→ `LivingEntity.getHealth()` → `SynchedEntityData.get(DATA_HEALTH_ID)`.
javap 26.3: `isAlive = !isRemoved() && getHealth() > 0.0F && !dead`;
`getHealth` on a non-ServerPlayer is exactly the DATA_HEALTH_ID read (the
ServerPlayer branch is unreachable for stands). The port serves health the
same way (`itemsById[O_ACC_HEALTH_ID].value as Float > 0.0f`), `dead` is a
protected LivingEntity field read through its single-class offset
(O_LE_DEAD), `isRemoved` stays the public method. A ported `alive == false`
still routes vanilla THROUGH the classification — the reduction cannot
mask a hurt/dying stand.

Both new resolutions are OPTIONAL try/catch ports: `offOf()` THROWS on
failure and would have killed the whole initTrackerLoopHandles tail (the
TT_REF tables, TTL_HANDLES_OK) with it.

## The observability rule

The `st=` counters joined stats() (`dispStand/dispStandMiss/
standPortHit/standPortMiss/STAND_MIRROR_ON|STAND_PORT`). The dead-port
verdict: an ungated counter is how a dead port shipped silently — the
hit/miss pair MUST be observable.

## Local verification (2-vCPU sandbox, parity spread leg)

SCALE=10000, PCT_ITEMS=0.6, PCT_STANDS=0.2, AREA=300, PARITY=1, seed 4242:

- module tests green (javac compiles the new block; weave structure tests
  8/8; the registered-natives table 1/1);
- ON-leg telemetry at the measure end: `st=710533/0/710533/0/11` —
  every mirrored stand took the port, ZERO misses, zero vanilla falls;
  `ttlA=378/9901/1` (the arena tracker unaffected);
- parity-compare PASS bit-identical on all 3 snapshots (T389/T639/final
  tags) against the vanilla leg.

## Expected effect

The three chains leave the profile: ~14.6% of the spread-100k-mod tick
becomes ~6 Unsafe loads per stand per tick (the same cost class the
probes already pay — the Unsafe.getObject leaf family). The spread
p50 expectation: 54.8 → ~47 ms (−14% relative) — the CI baseline's
paired spread-100k leg on 97a5d02 measures the honest number.

## NEXT (unchanged from the 25 addendum, re-ranked)

1. tickListForEach 5.46% (the dispatch walk body: arr load + instanceof +
   entId — the arena-ordered walk needs the tick-list order proof; the TL
   parity stream is the judge);
2. itemTickFlush 3.67% + the flush prologue (the epoch-counter redesign
   zeroes the quietPass sweep ~1.7 ms/tick);
3. the vanilla floor (~17%: chunk ticking, light, the world tick) is the
   −90% wall on items.
