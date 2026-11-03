# Baseline run: `mixed-150k`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110464 -> 110464) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 759.744 / 954.858 / 1543.110 / 1766.819 |
| MSPT mean, ms | 797.886 |
| TPS estimate (1000/p50) | 1.3 |
| heap used / max, MB | 2638.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 38.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 2.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.76% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.00% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 0.93% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 0.86% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 12 | 0.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 15 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |

