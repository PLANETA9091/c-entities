# Baseline run: `spread-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97372 -> 97372) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 289.203 / 472.705 / 621.785 / 841.465 |
| MSPT mean, ms | 323.516 |
| TPS estimate (1000/p50) | 3.5 |
| heap used / max, MB | 1964.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.84% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.71% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8633` |
| 3 | 3.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 4 | 2.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.60% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5528` |
| 7 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.76% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 9 | 1.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4737` |
| 11 | 1.22% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.13% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5383` |
| 13 | 1.09% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.03% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 15 | 0.92% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |

