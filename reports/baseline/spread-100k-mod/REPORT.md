# Baseline run: `spread-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97424 -> 97424) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 519.634 / 687.425 / 900.049 / 1151.854 |
| MSPT mean, ms | 554.918 |
| TPS estimate (1000/p50) | 1.9 |
| heap used / max, MB | 2832.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 3.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3401` |
| 3 | 3.45% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 411` |
| 4 | 2.30% | `java.util.ArrayList.elementData(int) line: 412` |
| 5 | 2.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10970` |
| 6 | 1.89% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |
| 7 | 1.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.31% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3345` |
| 10 | 1.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.13% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 0.97% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 0.95% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 0.95% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3375` |

