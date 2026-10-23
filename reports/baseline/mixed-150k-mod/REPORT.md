# Baseline run: `mixed-150k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 99865 -> 99842) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 463.862 / 510.632 / 567.209 / 608.343 |
| MSPT mean, ms | 463.559 |
| TPS estimate (1000/p50) | 2.2 |
| heap used / max, MB | 2136.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.79% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 2 | 4.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 2.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 1.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.85% | `java.util.HashMap.getNode(Object) line: 586` |
| 8 | 1.79% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 9 | 1.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1048` |
| 10 | 1.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.22% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 12 | 1.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 13 | 1.18% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 14 | 0.96% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.95% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |

