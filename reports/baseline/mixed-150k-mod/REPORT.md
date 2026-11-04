# Baseline run: `mixed-150k-mod`

Server: `26.3-240-6b68358 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109731 -> 109731) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 328.521 / 420.461 / 555.289 / 622.233 |
| MSPT mean, ms | 349.441 |
| TPS estimate (1000/p50) | 3.0 |
| heap used / max, MB | 1827.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.93% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 2 | 7.62% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 4.48% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 4 | 3.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 6 | 2.03% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 7 | 2.01% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 8 | 1.82% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10678` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 10 | 1.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.50% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1291` |
| 12 | 1.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10076` |
| 13 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.22% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2408` |
| 15 | 1.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |

