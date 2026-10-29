# Baseline run: `mixed-150k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109693 -> 109693) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 402.540 / 500.997 / 767.700 / 858.183 |
| MSPT mean, ms | 426.659 |
| TPS estimate (1000/p50) | 2.5 |
| heap used / max, MB | 1744.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8056` |
| 3 | 3.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5092` |
| 5 | 2.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.49% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8197` |
| 7 | 1.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4374` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 10 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.20% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4947` |
| 13 | 1.07% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.91% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 0.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |

