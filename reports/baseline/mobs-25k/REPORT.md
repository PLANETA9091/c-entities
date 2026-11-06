# Baseline run: `mobs-25k`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25061 -> 25061) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 403.114 / 452.814 / 493.450 / 519.778 |
| MSPT mean, ms | 412.676 |
| TPS estimate (1000/p50) | 2.5 |
| heap used / max, MB | 1845.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 32.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.46% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.22% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.13% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.03% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 14 | 0.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

