# 20. Retarget to Paper 26.3 (the only supported target)

Date: 2026-10-23. Directive: rework c-entities for **Minecraft 26.3** and
26.3 only (year-based versioning replaced 1.21.x in 2026; 26.3 is the latest
release — `26.4-snapshot-5` is already in piston-meta).

Everything below was **javap-verified on the shipped mojang-mapped
paper-26.3-218 jar** (build 218, main@1b9bf5c, Java 25).

## 1. Platform facts

| Fact | 1.21.8 | 26.3 |
|---|---|---|
| Required Java | 21 (class files v65) | **25** (`java-runtime-epsilon`, majorVersion 25, class files **v69**) |
| Bundled ASM | 9.8 | **9.9.1** (reads v69 → cplug-sdk's SdkAsmHelper, compiled vs the 9.8 *API*, runs unchanged) |
| paper-api artifact | `1.21.8-R0.1-SNAPSHOT` | **`26.3.build.218-stable`** (new `<mc>.build.<n>-<channel>` scheme) |
| paperclip layout | `versions/paper-1.21.8.jar` | `versions/26.3/paper-26.3.jar`; the mapped jar references fastutil/DFU/jspecify/jetbrains-annotations from the boot's `libraries/` (javac refuses to complete symbols without them → build.rs appends every `libraries/**/*.jar` to the javac classpath) |

CRUSSTY itself needed **zero changes**: the runtime/SDK pin stays a34d529.

## 2. Semantic parity of the item physics (the core result)

Normalized opcode-stream diffs (`scripts/disdiff.py`, `scripts/semdiff.py`)
of `ItemEntity` 1.21.8 vs 26.3:

- `tick()` — **semantically identical**: same gravity/fluid/merge-gate/
  despawn flow, same constants (1.0E-7 no-collision deflate, 9.999999747378752E-6
  move gate, 0.10000000149011612 fluid threshold, (tickCount+getId())%4).
  Only deltas: the y-drag literal `0.98` became `getAirDrag()` (which
  **returns 0.98f** — numerically equal), `Level.isClientSide` is now read
  via a method, and the branch layout shifted (compiler).
- `tryToMerge(ItemEntity)` — **identical** (merge parity intact: bigger
  absorbs smaller, equal → scanner dies).
- `mergeWithNeighbours()` — identical modulo a constant-pool index.
- `inactiveTick()` — identical modulo isClientSide field→method.

So `centities-core`'s bit-exact port stays valid **unchanged**.

## 3. Anchor changes the module needed (7 fixes)

| Site | 1.21.8 | 26.3 | Fix |
|---|---|---|---|
| `CeHooks` complex-block guard | `updateEntityMovementAfterFallOn` | renamed **`bounceOn`** (fall-damage rework; `Block.getBounceRestitution` appeared) | overrides check uses `"bounceOn"` |
| applier HAS_IMPULSE | `Entity.hasImpulse` | renamed **`Entity.needsSync`** (public) | `e.needsSync = true` |
| 7 sites | `Level.isClientSide` public field | **private** + `isClientSide()` | method call |
| dispatch fresh-gate | `e.portalCooldown` public | **private** | `e.isOnPortalCooldown()` |
| section hydrate | `LevelChunkSection.states` public | **private** | `section.getStates().get(x,y,z)` (PalettedContainer.get(III) intact) |
| wake probe | `Level.setBlock(pos,state,flags)` 3-arg existed | **removed** — only `setBlock(pos,state,flags,recursion)` 4-arg | probe now requires ≥1 of {4-arg, 3-arg} ("not found" tolerated per-overload) |
| bench plugin | `world.setTime(6000L)` | throws "Cannot set time in world without world clock" | guarded; `DO_DAYLIGHT_CYCLE=false` pins time |

Everything else verified unchanged: weave targets `ItemEntity.tick /
inactiveTick / mergeWithNeighbours`, `ServerLevel.tickNonPassenger(Entity)`,
`Level.getEntities(Entity,AABB,Predicate)` (moonrise mirror +
`PlatformHooks.addToGetEntities` body identical), `EntityLookup
(addEntity(Entity,ZZ)Z / removeEntity(Entity)V / getEntities(E,AABB,List,P))`
at the same `chunk_system/level/entity` FQN, `Entity.setBoundingBox(AABB)`,
`Entity.setDeltaMovement(Vec3)/(DDD)`, fields `fallDistance / tickCount /
onGround / stuckSpeedMultiplier / remainingFireTicks / despawnRate /
frictionState / despawnTime / pickupDelay / age`, `Block.stepOn / fallOn`,
`Level.paperConfig()`, `CraftEventFactory.callItemDespawnEvent`,
`EntityRemoveEvent$Cause`.

## 4. Harness changes

- **Workflows** (module/e2e/baseline/ab-paired): setup-java 21 → **25**,
  fetch 1.21.8 → 26.3, `paper-26.3.jar` / `paperclip-26.3.jar` names.
- **ViaVersion**: mineflayer/minecraft-data speaks at most **26.1**, so the
  bench bot cannot join a 26.3 server directly. CI downloads ViaVersion
  (Hangar API → hangarcdn URL) and run-baseline.sh stages it into
  `plugins/` for BOTH A/B legs (paired fairness preserved). bot.js speaks
  `version: '26.1'`.
- **26.3 command syntax (console-verified)**: console `execute in <dim>`
  rejects the position argument → e2e uses plain `summon` (default
  dimension); entity NBT keys are snake_case since 1.21.5
  (`no_ai`, `persistence_required`, `marker`), the item stack key stays
  `Item:{id:...,count:1}`. Without the bench plugin's chunk tickets any
  summoned entity is evicted within a tick (no entity-ticking chunks) —
  the plugin must be present for any summon experiment.
- pom.xml → paper-api `26.3.build.218-stable` (plugin compiled with JDK 25,
  `--release 21` bytecode loads on the 26.3 JVM).

## 5. Local e2e on 26.3 (pre-CI validation)

Boot Paper 26.3 + CRUSSTY runtime (a34d529) + c-entities module + ce-baseline:

- `pipeline ready`, `natives registered`, all patches rc=0 on v69 classes,
  **ARMED**, `Done (16.4s)`
- live counters: `add=36 rm=1 box=8680 flush=137/8528 fast=85 mirror=14493
  merge=20/20 disp=24295/517/504/71/28/r20 desync=0` — the woven fast path,
  merge fast path, flush and dispatch guard all serve real traffic.

Core tests 21/21 green; module patcher tests green against the real v69 jar
(own classfile parser accepted v69 without changes).

## 6. First honest A/B on 26.3 (CI, paired same-runner, mean MSPT)

| Scenario | vanilla | module | Δ | note |
|---|---:|---:|---:|---|
| items-100k | 65.8 | **35.1** | **−46.6%** | mass 100000→100000 both legs; flush avg 6.6ms, ok/ab=0 aborts |
| spread-100k | 327.8 | **270.8** | **−17.4%** | gate mostly routes mirror (fast 21µs > mirror 5µs there) |
| mobs-25k | 367.9 | **215.4** | **−41.4%** | query fusion serves mob AI (2.5M fast + 10.5M mirror) |
| mixed-150k | 534.7 | 616.6 | **+15.3%** | OPEN: 26.3 vanilla floor dropped (670→535 vs 1.21.8); 20.3M fast queries at EMA 11µs ≈ 84ms/tick; `CeHooks.worldId` 13.28% self-time in spread-mod hotspots (identity-memo suspicion / JFR inline attribution) — next lever |

Telemetry integrity on 26.3: `ab0 stuck0 unl0 fld0 cpx0 fal0` across every
flush (the 1.21.8 abort-storm class of bugs did not survive the retarget),
desync=0, ARMED in all mod legs.

Next levers (26.3-specific): cheapen the fast-path per-query price (JNI
candidate return, worldId memo), then the Java-floor (tracker/EntityTickList).
