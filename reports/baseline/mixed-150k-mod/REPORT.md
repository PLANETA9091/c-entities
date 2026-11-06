# Baseline run: `mixed-150k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109629 -> 109629) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 419.796 / 506.013 / 748.116 / 833.734 |
| MSPT mean, ms | 439.346 |
| TPS estimate (1000/p50) | 2.4 |
| heap used / max, MB | 2029.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2747` |
| 2 | 7.84% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10521` |
| 4 | 3.28% | `net.minecraft.world.entity.Entity.moonrise$getSectionZ() line: 463` |
| 5 | 2.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.65% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2450` |
| 7 | 2.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6883` |
| 8 | 1.94% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2307` |
| 9 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.43% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.16% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.15% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1293` |
| 14 | 1.12% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |

