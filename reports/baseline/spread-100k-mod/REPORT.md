# Baseline run: `spread-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97382 -> 97382) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 198.898 / 346.521 / 563.882 / 707.617 |
| MSPT mean, ms | 232.189 |
| TPS estimate (1000/p50) | 5.0 |
| heap used / max, MB | 1889.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4838` |
| 2 | 3.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7778` |
| 4 | 2.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.12% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7725` |
| 7 | 2.09% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7919` |
| 8 | 1.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.65% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 1.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4693` |
| 11 | 1.52% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 12 | 1.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4120` |
| 13 | 1.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

