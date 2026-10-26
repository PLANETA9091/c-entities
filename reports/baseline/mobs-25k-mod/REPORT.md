# Baseline run: `mobs-25k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25055 -> 25055) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 185.987 / 292.340 / 369.181 / 419.746 |
| MSPT mean, ms | 199.786 |
| TPS estimate (1000/p50) | 5.4 |
| heap used / max, MB | 1364.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 1.82% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], float[], Mob) line: 5885` |
| 5 | 1.81% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 6 | 1.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.38% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 8 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.34% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.10% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.10% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 12 | 0.98% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 13 | 0.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 0.82% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |

