# Baseline run: `mobs-25k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25049 -> 25049) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 198.765 / 355.292 / 574.137 / 622.378 |
| MSPT mean, ms | 228.636 |
| TPS estimate (1000/p50) | 5.0 |
| heap used / max, MB | 1451.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.30% | `java.util.HashMap.getNode(Object) line: 586` |
| 2 | 4.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.40% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1465` |
| 7 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.23% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7357` |
| 10 | 1.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 11 | 1.08% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.03% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 13 | 0.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 14 | 0.82% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6250` |
| 15 | 0.81% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |

