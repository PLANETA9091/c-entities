# Baseline run: `mixed-150k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109624 -> 109467) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 1344.140 / 1634.876 / 1890.589 / 2072.083 |
| MSPT mean, ms | 1374.670 |
| TPS estimate (1000/p50) | 0.7 |
| heap used / max, MB | 1793.9 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.get(int) line: 102` |
| 2 | 5.71% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 3 | 2.92% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 4 | 2.16% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 5 | 2.03% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.47% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.44% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 10 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 12 | 1.21% | `java.lang.Enum.ordinal() line: 128` |
| 13 | 1.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 14 | 1.14% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 15 | 1.12% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |

