# Baseline run: `mobs-25k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25067 -> 25067) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 261.360 / 454.521 / 695.094 / 724.281 |
| MSPT mean, ms | 295.529 |
| TPS estimate (1000/p50) | 3.8 |
| heap used / max, MB | 1480.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.54% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1484` |
| 4 | 2.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 7 | 1.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 11 | 1.11% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7357` |
| 12 | 1.08% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1465` |
| 13 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 15 | 0.98% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |

