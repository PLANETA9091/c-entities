# Baseline run: `mixed-150k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110433 -> 110421) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 1034.197 / 1101.770 / 1161.898 / 1252.512 |
| MSPT mean, ms | 1035.717 |
| TPS estimate (1000/p50) | 1.0 |
| heap used / max, MB | 3763.8 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 41.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.56% | `java.util.HashMap.getNode(Object) line: 585` |
| 5 | 1.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.12% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 7 | 1.03% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.01% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 0.92% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 10 | 0.92% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 11 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.84% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 14 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.67% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |

