# Baseline run: `items-100k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54660 -> 54660) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 20.116 / 33.059 / 81.410 / 109.966 |
| MSPT mean, ms | 23.797 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1073.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 9.60% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 4.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 4 | 3.08% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 2.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1550` |
| 6 | 2.02% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2192` |
| 7 | 1.74% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 8 | 1.65% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 9 | 1.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2099` |
| 11 | 1.40% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 12 | 1.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.11% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 14 | 1.05% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 15 | 1.03% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1589` |

