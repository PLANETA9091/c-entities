# Baseline run: `mixed-150k-mod`

Server: `26.3-242-fd28d65 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109729 -> 109729) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 390.629 / 486.790 / 622.869 / 788.841 |
| MSPT mean, ms | 411.484 |
| TPS estimate (1000/p50) | 2.6 |
| heap used / max, MB | 1914.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 8.40% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 4.75% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 4 | 2.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10303` |
| 5 | 2.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 7 | 1.95% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2399` |
| 8 | 1.71% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10939` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1292` |
| 10 | 1.48% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 13 | 1.28% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 14 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.99% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |

