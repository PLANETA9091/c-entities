# Baseline run: `mixed-150k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109686 -> 109686) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 411.392 / 508.944 / 668.867 / 780.471 |
| MSPT mean, ms | 433.430 |
| TPS estimate (1000/p50) | 2.4 |
| heap used / max, MB | 1907.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2747` |
| 2 | 10.42% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.25% | `net.minecraft.world.entity.Entity.moonrise$getSectionZ() line: 463` |
| 4 | 3.08% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10508` |
| 5 | 2.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.35% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 11144` |
| 7 | 1.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6883` |
| 8 | 1.60% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.34% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2450` |
| 10 | 1.20% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.15% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.11% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2307` |
| 13 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |

