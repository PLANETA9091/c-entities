# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25062 -> 25062) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 220.842 / 386.484 / 566.229 / 607.606 |
| MSPT mean, ms | 248.141 |
| TPS estimate (1000/p50) | 4.5 |
| heap used / max, MB | 1611.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.08% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.31% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.25% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.53% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 10 | 1.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 11 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 12 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.28% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 14 | 1.05% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.97% | `java.util.EnumMap.get(Object) line: 248` |

