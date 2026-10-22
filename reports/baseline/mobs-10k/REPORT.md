# Baseline run: `mobs-10k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 10000 (items 0 / stands 0 / zombies 10000) (merge shrink: 10028 -> 10026) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 113.588 / 124.455 / 157.623 / 168.081 |
| MSPT mean, ms | 114.919 |
| TPS estimate (1000/p50) | 8.8 |
| heap used / max, MB | 1733.5 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.60% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 3 | 2.24% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 1.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 6 | 1.52% | `java.util.EnumMap.get(Object) line: 247` |
| 7 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.31% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 10 | 1.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.20% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 12 | 1.12% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 13 | 1.09% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 14 | 1.08% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 15 | 1.05% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |

