# Baseline run: `mixed-50k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 50000 (items 34897 / stands 10136 / zombies 4967) (merge shrink: 41452 -> 41451) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 128.178 / 147.684 / 195.474 / 217.526 |
| MSPT mean, ms | 132.319 |
| TPS estimate (1000/p50) | 7.8 |
| heap used / max, MB | 1155.0 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.33% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 3.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.14% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 6 | 2.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 7 | 1.74% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.74% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 11 | 1.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 13 | 1.41% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 14 | 1.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.22% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |

