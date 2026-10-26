# Baseline run: `items-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54738 -> 54738) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 12.801 / 16.949 / 67.585 / 107.895 |
| MSPT mean, ms | 14.474 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1404.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.49% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6650` |
| 2 | 7.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2110` |
| 4 | 2.82% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 2.64% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 6 | 2.56% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 8 | 1.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2035` |
| 10 | 1.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 11 | 1.44% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 12 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2081` |
| 14 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3494` |
| 15 | 1.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

