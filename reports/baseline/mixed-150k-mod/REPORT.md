# Baseline run: `mixed-150k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109731 -> 109731) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 413.742 / 519.495 / 684.663 / 759.528 |
| MSPT mean, ms | 434.715 |
| TPS estimate (1000/p50) | 2.4 |
| heap used / max, MB | 1925.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2657` |
| 2 | 10.72% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 2.99% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2388` |
| 4 | 2.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 2.15% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10632` |
| 7 | 1.82% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 8 | 1.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6725` |
| 9 | 1.30% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2386` |
| 10 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.14% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10030` |
| 12 | 1.11% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2397` |
| 13 | 1.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2625` |
| 14 | 1.05% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.02% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |

