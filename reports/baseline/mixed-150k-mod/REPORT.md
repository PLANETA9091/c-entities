# Baseline run: `mixed-150k-mod`

Server: `26.3-239-13565bc (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109713 -> 109713) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 300.264 / 384.067 / 501.572 / 657.433 |
| MSPT mean, ms | 317.377 |
| TPS estimate (1000/p50) | 3.3 |
| heap used / max, MB | 2108.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 2 | 3.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.13% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 2.74% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 5 | 2.28% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 6 | 1.77% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 7 | 1.67% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1291` |
| 8 | 1.60% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10074` |
| 10 | 1.48% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2408` |
| 11 | 1.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 13 | 1.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 14 | 1.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

