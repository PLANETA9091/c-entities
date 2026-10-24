# Baseline run: `mobs-25k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25062 -> 25062) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 207.434 / 341.345 / 425.224 / 506.898 |
| MSPT mean, ms | 215.858 |
| TPS estimate (1000/p50) | 4.8 |
| heap used / max, MB | 1541.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.17% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1467` |
| 3 | 1.89% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 4 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.46% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 6 | 1.41% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.20% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.12% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.get(VarHandle, Object) line: 92` |
| 10 | 1.06% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.05% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3332` |
| 12 | 1.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.99% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.96% | `java.util.EnumMap.get(Object) line: 248` |

