# Baseline run: `items-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54811 -> 54811) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.159 / 44.026 / 130.217 / 262.535 |
| MSPT mean, ms | 35.145 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1386.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 8.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 7.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 5.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 5 | 5.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 3.94% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 3.64% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 8 | 2.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1885` |
| 9 | 2.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2420` |
| 10 | 1.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1871` |
| 11 | 1.69% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1982` |
| 12 | 1.49% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 13 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.14% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 15 | 0.96% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |

