# Baseline run: `spread-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97502 -> 97502) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 170.496 / 280.681 / 405.109 / 550.672 |
| MSPT mean, ms | 192.763 |
| TPS estimate (1000/p50) | 5.9 |
| heap used / max, MB | 2664.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.55% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4932` |
| 3 | 2.87% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7872` |
| 6 | 2.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.21% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7819` |
| 8 | 2.00% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4787` |
| 9 | 1.80% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8013` |
| 10 | 1.70% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |

