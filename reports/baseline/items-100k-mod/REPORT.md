# Baseline run: `items-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54807 -> 54807) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 14.389 / 18.451 / 71.920 / 105.321 |
| MSPT mean, ms | 16.225 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1213.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.95% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 2 | 6.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2107` |
| 4 | 3.35% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4004` |
| 5 | 3.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 6 | 2.41% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 1.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2078` |
| 10 | 1.68% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 11 | 1.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3491` |
| 14 | 1.30% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |

