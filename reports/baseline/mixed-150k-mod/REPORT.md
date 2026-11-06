# Baseline run: `mixed-150k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109710 -> 109710) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 343.547 / 431.212 / 561.552 / 663.397 |
| MSPT mean, ms | 361.827 |
| TPS estimate (1000/p50) | 2.9 |
| heap used / max, MB | 2477.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2747` |
| 2 | 7.95% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 4.28% | `net.minecraft.world.entity.Entity.moonrise$getSectionZ() line: 463` |
| 4 | 3.55% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.01% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10491` |
| 6 | 2.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6883` |
| 7 | 2.21% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 11127` |
| 8 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2307` |
| 10 | 1.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2450` |
| 13 | 1.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.06% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

