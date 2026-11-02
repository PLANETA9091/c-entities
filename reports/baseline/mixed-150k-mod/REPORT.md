# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109696 -> 109696) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 292.568 / 411.706 / 610.188 / 729.376 |
| MSPT mean, ms | 323.726 |
| TPS estimate (1000/p50) | 3.4 |
| heap used / max, MB | 1643.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9193` |
| 2 | 3.72% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.38% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9334` |
| 4 | 3.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6152` |
| 5 | 2.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.52% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 2.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5416` |
| 8 | 2.06% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9107` |
| 9 | 1.60% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5998` |
| 10 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.11% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.07% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |

