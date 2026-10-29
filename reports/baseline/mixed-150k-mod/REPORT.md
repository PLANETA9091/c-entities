# Baseline run: `mixed-150k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109763 -> 109763) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 362.896 / 454.717 / 661.219 / 710.898 |
| MSPT mean, ms | 381.670 |
| TPS estimate (1000/p50) | 2.8 |
| heap used / max, MB | 1868.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.05% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5020` |
| 4 | 3.14% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8125` |
| 5 | 3.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7984` |
| 6 | 3.02% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 1.76% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7931` |
| 9 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.59% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4875` |
| 11 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4302` |
| 13 | 1.24% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.11% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

