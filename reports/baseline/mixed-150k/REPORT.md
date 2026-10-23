# Baseline run: `mixed-150k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110370 -> 110358) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 670.132 / 765.956 / 840.514 / 1426.984 |
| MSPT mean, ms | 674.624 |
| TPS estimate (1000/p50) | 1.5 |
| heap used / max, MB | 2282.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 30.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 2.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.53% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 1.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.04% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 0.97% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 13 | 0.91% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 14 | 0.89% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 15 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |

