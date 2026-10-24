# Baseline run: `mobs-25k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25075 -> 25075) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 169.225 / 314.175 / 391.342 / 436.663 |
| MSPT mean, ms | 184.803 |
| TPS estimate (1000/p50) | 5.9 |
| heap used / max, MB | 1678.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 1.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 1.69% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 3 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 1.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 1.52% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1420` |
| 6 | 1.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.32% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 8 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.19% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 141` |
| 11 | 1.16% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.15% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 14 | 1.12% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.09% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |

