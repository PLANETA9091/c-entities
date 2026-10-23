# Baseline run: `items-100k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54825 -> 54825) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.309 / 56.732 / 145.792 / 188.078 |
| MSPT mean, ms | 41.025 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1310.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 2 | 9.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 5.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 4.79% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 4.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 6 | 4.27% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 7 | 2.54% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1661` |
| 8 | 2.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 2.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1564` |
| 10 | 2.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2099` |
| 11 | 1.57% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 12 | 1.18% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 13 | 1.05% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.05% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 15 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

