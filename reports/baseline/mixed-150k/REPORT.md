# Baseline run: `mixed-150k`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110268 -> 110268) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 492.349 / 593.374 / 794.874 / 853.041 |
| MSPT mean, ms | 508.448 |
| TPS estimate (1000/p50) | 2.0 |
| heap used / max, MB | 2930.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 29.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 3 | 3.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.70% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 2.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.58% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 9 | 1.53% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 0.97% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 13 | 0.85% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 14 | 0.85% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 15 | 0.79% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |

