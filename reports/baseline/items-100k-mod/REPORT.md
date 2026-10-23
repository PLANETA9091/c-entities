# Baseline run: `items-100k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 82226 -> 82226) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 61.051 / 87.402 / 153.767 / 239.967 |
| MSPT mean, ms | 67.187 |
| TPS estimate (1000/p50) | 16.4 |
| heap used / max, MB | 1143.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 2 | 7.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 7.25% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 6.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 5 | 5.42% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2186` |
| 6 | 4.93% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 4.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2093` |
| 8 | 4.09% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 9 | 2.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1558` |
| 10 | 2.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1544` |
| 11 | 2.10% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1609` |
| 12 | 2.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1583` |
| 13 | 1.90% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 14 | 1.61% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 15 | 1.04% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |

