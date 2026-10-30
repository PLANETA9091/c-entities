# Baseline run: `spread-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97438 -> 97438) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 286.334 / 446.470 / 664.743 / 820.268 |
| MSPT mean, ms | 322.327 |
| TPS estimate (1000/p50) | 3.5 |
| heap used / max, MB | 1908.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.15% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8734` |
| 2 | 3.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8593` |
| 3 | 2.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.79% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5629` |
| 6 | 2.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.88% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8540` |
| 10 | 1.55% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5484` |
| 11 | 1.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4838` |
| 13 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.03% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.02% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |

