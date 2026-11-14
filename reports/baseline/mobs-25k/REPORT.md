# Baseline run: `mobs-25k`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24293 -> 24293) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 543.203 / 620.243 / 683.857 / 812.244 |
| MSPT mean, ms | 553.653 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 1082.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 3 | 2.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.22% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.00% | `java.util.HashMap.getNode(Object) line: 586` |
| 7 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 0.97% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 9 | 0.96% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4084` |
| 10 | 0.84% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.57% | `java.util.EnumMap.get(Object) line: 248` |

