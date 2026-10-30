# Baseline run: `mixed-150k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109744 -> 109744) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 259.727 / 382.464 / 514.966 / 672.236 |
| MSPT mean, ms | 286.531 |
| TPS estimate (1000/p50) | 3.9 |
| heap used / max, MB | 1812.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.53% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8717` |
| 3 | 3.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5753` |
| 4 | 3.02% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.99% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8858` |
| 6 | 2.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4962` |
| 8 | 2.00% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8664` |
| 9 | 1.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5608` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 13 | 1.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |

