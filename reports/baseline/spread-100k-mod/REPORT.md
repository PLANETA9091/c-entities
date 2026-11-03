# Baseline run: `spread-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97492 -> 97492) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 430.412 / 563.525 / 705.757 / 905.918 |
| MSPT mean, ms | 451.226 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 2777.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 2 | 5.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2945` |
| 3 | 4.81% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2515` |
| 4 | 3.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2983` |
| 5 | 3.04% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10490` |
| 6 | 2.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6583` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2957` |
| 10 | 1.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2928` |
| 11 | 1.08% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9888` |
| 12 | 0.93% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2246` |
| 13 | 0.92% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 14 | 0.82% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 15 | 0.81% | `java.util.ArrayDeque.pollFirst() line: 378` |

