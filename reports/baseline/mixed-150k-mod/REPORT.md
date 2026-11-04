# Baseline run: `mixed-150k-mod`

Server: `26.3-241-5050b0f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109871 -> 109871) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 401.916 / 505.193 / 707.330 / 783.328 |
| MSPT mean, ms | 429.768 |
| TPS estimate (1000/p50) | 2.5 |
| heap used / max, MB | 1947.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 7.34% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.25% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 4 | 3.21% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10175` |
| 5 | 2.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.27% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 7 | 1.96% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 8 | 1.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 9 | 1.34% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1292` |
| 10 | 1.31% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2399` |
| 11 | 1.28% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10811` |
| 12 | 1.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.21% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

