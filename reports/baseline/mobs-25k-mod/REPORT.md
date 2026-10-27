# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25060 -> 25060) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 174.463 / 298.726 / 477.986 / 552.652 |
| MSPT mean, ms | 197.024 |
| TPS estimate (1000/p50) | 5.7 |
| heap used / max, MB | 1359.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.91% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.18% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 1.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.58% | `java.util.EnumMap.get(Object) line: 248` |
| 7 | 1.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.32% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 9 | 1.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 10 | 1.18% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7685` |
| 11 | 1.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 13 | 1.07% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 14 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 15 | 1.04% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

