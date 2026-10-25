# Baseline run: `mobs-25k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25051 -> 25051) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 193.222 / 328.644 / 422.826 / 465.646 |
| MSPT mean, ms | 208.462 |
| TPS estimate (1000/p50) | 5.2 |
| heap used / max, MB | 1915.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 1.78% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 1.46% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5463` |
| 4 | 1.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.41% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], float[], Mob) line: 4887` |
| 6 | 1.40% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.32% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 5492` |
| 8 | 1.26% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 9 | 1.24% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 1.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.98% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 14 | 0.93% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 15 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

