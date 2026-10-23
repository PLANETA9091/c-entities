# Baseline run: `mobs-25k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 19967 -> 19934) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 192.965 / 236.525 / 272.259 / 319.863 |
| MSPT mean, ms | 201.980 |
| TPS estimate (1000/p50) | 5.2 |
| heap used / max, MB | 1745.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 1.50% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 5 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 1.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 7 | 1.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.21% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.10% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 1.01% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 0.99% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 14 | 0.97% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 0.95% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |

