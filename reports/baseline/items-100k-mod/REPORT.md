# Baseline run: `items-100k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54807 -> 54807) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 33.034 / 50.910 / 166.914 / 214.108 |
| MSPT mean, ms | 39.670 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1479.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 9.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 5.77% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 4.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 4.21% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 6 | 4.18% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 3.05% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1661` |
| 8 | 2.89% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 9 | 1.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2099` |
| 10 | 1.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1564` |
| 11 | 1.15% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 12 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.74% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 15 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

