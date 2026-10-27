# Baseline run: `mobs-25k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25054 -> 25054) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 168.577 / 256.690 / 336.559 / 388.980 |
| MSPT mean, ms | 181.964 |
| TPS estimate (1000/p50) | 5.9 |
| heap used / max, MB | 1564.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.57% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.12% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 1.57% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7145` |
| 8 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 0.99% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1414` |
| 10 | 0.98% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 11 | 0.92% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 0.92% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 13 | 0.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 15 | 0.91% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7281` |

