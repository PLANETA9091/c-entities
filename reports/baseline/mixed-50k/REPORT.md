# Baseline run: `mixed-50k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 50000 (items 34897 / stands 10136 / zombies 4967) (merge shrink: 41475 -> 41475) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 200.511 / 224.626 / 258.683 / 281.592 |
| MSPT mean, ms | 204.713 |
| TPS estimate (1000/p50) | 5.0 |
| heap used / max, MB | 1320.6 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 20.63% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.21% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 7 | 1.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 8 | 1.26% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 9 | 1.21% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 10 | 1.01% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 11 | 0.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 0.96% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 14 | 0.96% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 15 | 0.94% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |

