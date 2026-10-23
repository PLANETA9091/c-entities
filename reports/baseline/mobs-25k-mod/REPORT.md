# Baseline run: `mobs-25k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25071 -> 25071) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 117.858 / 183.640 / 270.298 / 350.507 |
| MSPT mean, ms | 123.834 |
| TPS estimate (1000/p50) | 8.5 |
| heap used / max, MB | 1873.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.62% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 3 | 2.22% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 4 | 1.89% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 1.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 7 | 1.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 8 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 12 | 1.13% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 720` |
| 13 | 1.09% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.08% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.getX() line: 5094` |

