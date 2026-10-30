# Baseline run: `spread-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97374 -> 97374) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 273.077 / 414.122 / 506.441 / 591.730 |
| MSPT mean, ms | 298.835 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 2040.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.48% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.37% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8840` |
| 3 | 3.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8699` |
| 4 | 2.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.41% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5735` |
| 8 | 2.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8646` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4944` |
| 11 | 1.37% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5590` |
| 14 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.02% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |

