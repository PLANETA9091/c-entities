# Baseline run: `mobs-25k-native`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24182 -> 23763) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 394.557 / 475.026 / 569.748 / 754.510 |
| MSPT mean, ms | 407.352 |
| TPS estimate (1000/p50) | 2.5 |
| heap used / max, MB | 1493.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 31.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 2.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 7 | 1.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 8 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.16% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.96% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 12 | 0.85% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |

