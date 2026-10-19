# Baseline run: `mobs-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25073 -> 25067) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 527.597 / 571.523 / 608.136 / 735.582 |
| MSPT mean, ms | 531.570 |
| TPS estimate (1000/p50) | 1.9 |
| heap used / max, MB | 1537.1 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 21.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 3 | 3.47% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.49% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 7 | 1.15% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.07% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 9 | 0.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 10 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 0.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 13 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.60% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 15 | 0.60% | `java.util.EnumMap.get(Object) line: 247` |

