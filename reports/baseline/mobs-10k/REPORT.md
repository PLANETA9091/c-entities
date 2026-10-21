# Baseline run: `mobs-10k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 10000 (items 0 / stands 0 / zombies 10000) (merge shrink: 10026 -> 10024) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 110.288 / 122.489 / 156.393 / 182.183 |
| MSPT mean, ms | 112.468 |
| TPS estimate (1000/p50) | 9.1 |
| heap used / max, MB | 1425.8 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.50% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 4 | 2.88% | `java.util.HashMap.getNode(Object) line: 585` |
| 5 | 1.82% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 6 | 1.75% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.35% | `java.util.EnumMap.get(Object) line: 247` |
| 9 | 1.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 13 | 1.04% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 14 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 1.01% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |

