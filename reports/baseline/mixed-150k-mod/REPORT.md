# Baseline run: `mixed-150k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109739 -> 109739) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 393.170 / 498.525 / 758.732 / 858.968 |
| MSPT mean, ms | 417.253 |
| TPS estimate (1000/p50) | 2.5 |
| heap used / max, MB | 1777.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.26% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8042` |
| 3 | 2.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5078` |
| 5 | 2.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4360` |
| 7 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.60% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8183` |
| 9 | 1.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7989` |
| 10 | 1.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.14% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4933` |
| 13 | 1.11% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.03% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |

