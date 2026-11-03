# Baseline run: `mixed-150k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109773 -> 109773) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 372.771 / 495.360 / 697.421 / 775.689 |
| MSPT mean, ms | 406.320 |
| TPS estimate (1000/p50) | 2.7 |
| heap used / max, MB | 1921.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.69% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 2 | 8.14% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.00% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.35% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 6 | 1.96% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10643` |
| 7 | 1.81% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2408` |
| 8 | 1.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6736` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 10 | 1.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10041` |
| 11 | 1.29% | `java.util.ArrayList.elementData(int) line: 412` |
| 12 | 1.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.19% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5973` |
| 15 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

