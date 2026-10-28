# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25063 -> 25063) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 284.122 / 479.888 / 724.744 / 808.367 |
| MSPT mean, ms | 325.160 |
| TPS estimate (1000/p50) | 3.5 |
| heap used / max, MB | 1413.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.63% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.99% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.83% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 4 | 2.16% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.83% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 7 | 1.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 10 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 12 | 1.01% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 13 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 14 | 0.87% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |

