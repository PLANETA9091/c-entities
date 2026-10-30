# Baseline run: `mixed-150k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109725 -> 109725) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 292.150 / 412.281 / 518.744 / 566.882 |
| MSPT mean, ms | 318.240 |
| TPS estimate (1000/p50) | 3.4 |
| heap used / max, MB | 1701.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.75% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8696` |
| 3 | 3.36% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.05% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8837` |
| 5 | 3.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5732` |
| 6 | 2.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4941` |
| 8 | 1.95% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8643` |
| 9 | 1.54% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5587` |
| 10 | 1.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.16% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |

