# Baseline run: `spread-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97441 -> 97441) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 304.796 / 366.805 / 547.943 / 606.173 |
| MSPT mean, ms | 325.637 |
| TPS estimate (1000/p50) | 3.3 |
| heap used / max, MB | 1913.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.60% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.46% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 6 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 10 | 1.35% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 11 | 1.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.15% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.12% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 1.12% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |

