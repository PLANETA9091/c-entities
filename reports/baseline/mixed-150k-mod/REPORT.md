# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109760 -> 109760) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 312.127 / 418.375 / 656.201 / 738.539 |
| MSPT mean, ms | 340.995 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 1763.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.23% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9008` |
| 3 | 3.32% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6031` |
| 5 | 2.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5295` |
| 7 | 2.12% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8955` |
| 8 | 1.68% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5877` |
| 10 | 1.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.22% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.05% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |

