# Baseline run: `mixed-150k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110280 -> 110280) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 678.261 / 813.220 / 1146.533 / 1265.677 |
| MSPT mean, ms | 705.620 |
| TPS estimate (1000/p50) | 1.5 |
| heap used / max, MB | 2250.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.01% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 1.88% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 1.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.04% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 10 | 1.01% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.00% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 13 | 0.85% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |

