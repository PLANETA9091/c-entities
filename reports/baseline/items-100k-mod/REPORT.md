# Baseline run: `items-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54886 -> 54886) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.231 / 50.732 / 122.886 / 176.326 |
| MSPT mean, ms | 35.374 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 996.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 6.72% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 3 | 6.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 5.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 5 | 5.41% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 2.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1888` |
| 7 | 2.07% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1953` |
| 8 | 2.00% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2530` |
| 9 | 1.65% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 10 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2437` |
| 11 | 1.06% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 12 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.00% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 14 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.96% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |

