# Baseline run: `mixed-150k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109754 -> 109754) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 259.398 / 372.575 / 506.590 / 615.277 |
| MSPT mean, ms | 283.982 |
| TPS estimate (1000/p50) | 3.9 |
| heap used / max, MB | 1835.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.20% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 3 | 3.23% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 4 | 3.07% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 6 | 2.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.14% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 8 | 2.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 10 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.44% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 12 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.18% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 15 | 1.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |

