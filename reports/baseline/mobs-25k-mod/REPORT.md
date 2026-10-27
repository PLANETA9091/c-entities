# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25059 -> 25059) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 186.666 / 306.509 / 484.325 / 578.575 |
| MSPT mean, ms | 209.185 |
| TPS estimate (1000/p50) | 5.4 |
| heap used / max, MB | 1836.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.15% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.20% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.11% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 7 | 1.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7525` |
| 8 | 1.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4585` |
| 9 | 1.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.13% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 11 | 1.13% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 12 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.07% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.06% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1759` |

