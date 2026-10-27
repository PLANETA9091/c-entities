# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62911 -> 62911) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 12.246 / 16.799 / 67.657 / 92.898 |
| MSPT mean, ms | 15.073 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1442.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.70% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 2 | 5.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 3 | 4.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2492` |
| 4 | 3.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 2.49% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2417` |
| 8 | 1.80% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 9 | 1.59% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 10 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2463` |
| 11 | 1.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 12 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3895` |

