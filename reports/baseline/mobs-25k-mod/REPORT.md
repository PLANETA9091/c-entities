# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25059 -> 25059) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 159.818 / 265.054 / 476.388 / 533.872 |
| MSPT mean, ms | 181.153 |
| TPS estimate (1000/p50) | 6.3 |
| heap used / max, MB | 1489.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.62% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 2.49% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 6 | 1.93% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 7 | 1.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.50% | `java.util.EnumMap.get(Object) line: 248` |
| 9 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4614` |
| 11 | 1.39% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 12 | 1.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7554` |
| 13 | 1.33% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7695` |
| 14 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |

