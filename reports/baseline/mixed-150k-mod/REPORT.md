# Baseline run: `mixed-150k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110281 -> 110264) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 717.872 / 809.159 / 928.371 / 1004.288 |
| MSPT mean, ms | 724.919 |
| TPS estimate (1000/p50) | 1.4 |
| heap used / max, MB | 2332.2 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 32.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.78% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 2.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.83% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 8 | 1.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.18% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 10 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 11 | 0.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 0.93% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 14 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |

