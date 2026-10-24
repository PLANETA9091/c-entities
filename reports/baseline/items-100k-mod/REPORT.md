# Baseline run: `items-100k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54842 -> 54842) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 31.145 / 46.469 / 122.014 / 167.456 |
| MSPT mean, ms | 37.200 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1253.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 16.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 7.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 3 | 7.39% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 5.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 5.09% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 6 | 4.37% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 3.11% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 8 | 2.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1851` |
| 9 | 1.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1837` |
| 10 | 1.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2386` |
| 11 | 1.56% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1948` |
| 12 | 1.04% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 13 | 0.94% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 14 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 0.87% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |

