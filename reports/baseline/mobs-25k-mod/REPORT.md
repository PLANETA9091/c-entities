# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25044 -> 25044) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 255.946 / 423.767 / 590.401 / 673.813 |
| MSPT mean, ms | 284.745 |
| TPS estimate (1000/p50) | 3.9 |
| heap used / max, MB | 1278.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.33% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.26% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 3.08% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 4 | 2.89% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 5 | 2.11% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.74% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 11 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.22% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 14 | 1.03% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |

