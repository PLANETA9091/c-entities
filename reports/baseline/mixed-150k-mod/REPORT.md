# Baseline run: `mixed-150k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100594 -> 100594) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 308.784 / 409.615 / 570.461 / 663.077 |
| MSPT mean, ms | 328.480 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 1833.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 3 | 2.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.36% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7357` |
| 5 | 2.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 1.97% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1484` |
| 9 | 1.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.33% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4131` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 14 | 1.12% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

