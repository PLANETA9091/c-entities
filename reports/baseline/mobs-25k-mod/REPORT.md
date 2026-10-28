# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25068 -> 25068) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 230.706 / 447.332 / 655.340 / 760.241 |
| MSPT mean, ms | 263.835 |
| TPS estimate (1000/p50) | 4.3 |
| heap used / max, MB | 1311.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.62% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 1.72% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.31% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7837` |
| 6 | 1.31% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7770` |
| 7 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.16% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 9 | 1.14% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.14% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.13% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.08% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1870` |
| 14 | 1.07% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7629` |
| 15 | 1.05% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |

