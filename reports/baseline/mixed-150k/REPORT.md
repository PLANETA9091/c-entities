# Baseline run: `mixed-150k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110367 -> 110360) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 638.819 / 704.365 / 770.848 / 844.497 |
| MSPT mean, ms | 640.883 |
| TPS estimate (1000/p50) | 1.6 |
| heap used / max, MB | 2095.0 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 26.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 3.79% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.09% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.61% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.43% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 11 | 1.07% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.07% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 13 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 14 | 0.90% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |

