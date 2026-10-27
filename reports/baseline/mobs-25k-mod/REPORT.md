# Baseline run: `mobs-25k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25049 -> 25049) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 184.813 / 276.258 / 362.057 / 393.498 |
| MSPT mean, ms | 193.137 |
| TPS estimate (1000/p50) | 5.4 |
| heap used / max, MB | 1469.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.76% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.85% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 4 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.45% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.41% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7317` |
| 8 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7176` |
| 11 | 0.94% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 0.90% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 13 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1445` |
| 14 | 0.88% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6210` |
| 15 | 0.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |

