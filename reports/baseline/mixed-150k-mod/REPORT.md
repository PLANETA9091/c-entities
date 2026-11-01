# Baseline run: `mixed-150k-mod`

Server: `26.3-235-bb10cf0 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109642 -> 109642) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 257.328 / 353.366 / 472.280 / 570.260 |
| MSPT mean, ms | 278.802 |
| TPS estimate (1000/p50) | 3.9 |
| heap used / max, MB | 1839.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.87% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.05% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 3 | 3.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.98% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8977` |
| 5 | 2.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 6 | 2.63% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 9 | 2.21% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5727` |
| 10 | 1.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 12 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5145` |
| 14 | 1.19% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 15 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |

