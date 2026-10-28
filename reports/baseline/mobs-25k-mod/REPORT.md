# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25058 -> 25058) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 262.563 / 454.197 / 667.320 / 718.770 |
| MSPT mean, ms | 296.650 |
| TPS estimate (1000/p50) | 3.8 |
| heap used / max, MB | 1240.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.78% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.51% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 5 | 1.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.10% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 11 | 1.06% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 12 | 1.06% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 13 | 0.96% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 0.94% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |

