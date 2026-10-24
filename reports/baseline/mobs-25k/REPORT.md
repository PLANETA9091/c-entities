# Baseline run: `mobs-25k`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25049 -> 25049) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 358.919 / 405.946 / 446.522 / 475.740 |
| MSPT mean, ms | 368.144 |
| TPS estimate (1000/p50) | 2.8 |
| heap used / max, MB | 995.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 23.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 5.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 5 | 2.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 6 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.23% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.10% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 1.09% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.94% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 0.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.77% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.76% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |

