# Baseline run: `mixed-150k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109655 -> 109655) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 269.418 / 376.464 / 483.027 / 533.882 |
| MSPT mean, ms | 294.961 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 1840.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.99% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.61% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8729` |
| 3 | 3.55% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8535` |
| 4 | 3.34% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8588` |
| 6 | 2.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5624` |
| 7 | 2.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4833` |
| 9 | 1.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 10 | 1.46% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5479` |
| 11 | 1.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.14% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |

