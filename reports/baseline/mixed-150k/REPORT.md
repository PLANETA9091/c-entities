# Baseline run: `mixed-150k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110331 -> 110323) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 773.454 / 847.409 / 916.702 / 978.240 |
| MSPT mean, ms | 773.112 |
| TPS estimate (1000/p50) | 1.3 |
| heap used / max, MB | 1754.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 31.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 2.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.97% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.81% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 9 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.11% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 11 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.89% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 14 | 0.83% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 0.80% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |

