# Baseline run: `mixed-150k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100370 -> 100345) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 631.746 / 802.070 / 985.413 / 1151.062 |
| MSPT mean, ms | 652.397 |
| TPS estimate (1000/p50) | 1.6 |
| heap used / max, MB | 3201.7 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 31.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.54% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 2.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.61% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 1.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 8 | 1.36% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.29% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 10 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 11 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.93% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 13 | 0.84% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 14 | 0.81% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 15 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |

