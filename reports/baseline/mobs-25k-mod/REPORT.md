# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25057 -> 25057) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 198.653 / 320.036 / 509.106 / 627.467 |
| MSPT mean, ms | 219.595 |
| TPS estimate (1000/p50) | 5.0 |
| heap used / max, MB | 1422.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.79% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.32% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.30% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 6 | 1.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1944` |
| 7 | 1.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1963` |
| 8 | 1.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.53% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 1.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 11 | 1.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 12 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.22% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

