# Baseline run: `items-100k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54708 -> 54708) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.631 / 55.642 / 156.013 / 231.683 |
| MSPT mean, ms | 41.126 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1166.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.19% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 10.10% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 6.42% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 5.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 4.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 3.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 7 | 3.08% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1948` |
| 8 | 2.43% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 9 | 1.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2386` |
| 10 | 1.03% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 11 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.74% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 13 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 0.65% | `net.minecraft.world.entity.Entity.getType() line: 633` |

