# Baseline run: `mobs-25k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 20022 -> 20001) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 194.554 / 226.363 / 264.385 / 295.406 |
| MSPT mean, ms | 200.247 |
| TPS estimate (1000/p50) | 5.1 |
| heap used / max, MB | 1561.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 1.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 1.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 1.48% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.40% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 8 | 1.35% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1325` |
| 9 | 1.33% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 10 | 1.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1251` |
| 11 | 1.17% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.12% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 1.06% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.01% | `java.util.EnumMap.get(Object) line: 248` |

