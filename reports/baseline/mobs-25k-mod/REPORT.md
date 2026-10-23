# Baseline run: `mobs-25k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 20081 -> 20057) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 196.741 / 238.505 / 275.311 / 302.296 |
| MSPT mean, ms | 205.257 |
| TPS estimate (1000/p50) | 5.1 |
| heap used / max, MB | 1079.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 1.97% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 1.56% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 1.47% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 7 | 1.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 9 | 1.20% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 1.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 11 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 12 | 1.12% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1325` |
| 13 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 14 | 1.03% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

