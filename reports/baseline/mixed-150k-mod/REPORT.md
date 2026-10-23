# Baseline run: `mixed-150k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 99891 -> 99868) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 396.749 / 452.881 / 498.973 / 559.652 |
| MSPT mean, ms | 403.698 |
| TPS estimate (1000/p50) | 2.5 |
| heap used / max, MB | 1752.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.85% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 2 | 5.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 3.15% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 2.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 2.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.76% | `java.util.HashMap.getNode(Object) line: 586` |
| 9 | 1.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1048` |
| 10 | 1.57% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 11 | 1.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.48% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 13 | 1.37% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 14 | 1.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 1.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |

