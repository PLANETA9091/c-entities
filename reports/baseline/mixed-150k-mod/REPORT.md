# Baseline run: `mixed-150k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109719 -> 109719) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 393.098 / 493.211 / 653.900 / 734.802 |
| MSPT mean, ms | 408.989 |
| TPS estimate (1000/p50) | 2.5 |
| heap used / max, MB | 1836.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7872` |
| 3 | 2.81% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8013` |
| 4 | 2.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4932` |
| 5 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4214` |
| 9 | 1.63% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7819` |
| 10 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.18% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4787` |
| 13 | 1.17% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.01% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 15 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

