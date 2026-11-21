# Baseline run: `mobs-25k`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24176 -> 23757) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 374.201 / 446.051 / 546.166 / 637.350 |
| MSPT mean, ms | 383.964 |
| TPS estimate (1000/p50) | 2.7 |
| heap used / max, MB | 1037.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 28.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.76% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 2.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 1.89% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.30% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 8 | 1.23% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.09% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 0.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 0.79% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 13 | 0.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.76% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

