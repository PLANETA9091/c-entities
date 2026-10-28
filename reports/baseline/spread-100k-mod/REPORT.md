# Baseline run: `spread-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97421 -> 97421) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 167.816 / 290.346 / 456.839 / 555.795 |
| MSPT mean, ms | 195.861 |
| TPS estimate (1000/p50) | 6.0 |
| heap used / max, MB | 1865.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.89% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4879` |
| 2 | 3.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7819` |
| 4 | 3.46% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7960` |
| 5 | 2.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 2.09% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7766` |
| 8 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4734` |
| 10 | 1.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4161` |

