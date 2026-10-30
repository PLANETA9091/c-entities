# Baseline run: `spread-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97359 -> 97359) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 328.808 / 475.717 / 610.121 / 788.889 |
| MSPT mean, ms | 354.571 |
| TPS estimate (1000/p50) | 3.0 |
| heap used / max, MB | 1990.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.99% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8832` |
| 2 | 3.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 3.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8691` |
| 4 | 2.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5727` |
| 5 | 2.34% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.04% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.67% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8638` |
| 10 | 1.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4936` |
| 11 | 1.30% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 12 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.11% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.08% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5582` |
| 15 | 1.07% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |

