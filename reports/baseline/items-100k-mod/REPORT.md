# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62949 -> 62949) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 36.616 / 89.898 / 178.321 / 201.428 |
| MSPT mean, ms | 46.390 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1103.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.12% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 2 | 7.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 3 | 5.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 4 | 4.59% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.71% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2722` |
| 6 | 3.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 7 | 3.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2671` |
| 8 | 2.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4784` |
| 9 | 2.58% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2769` |
| 10 | 1.45% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 11 | 1.41% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4079` |
| 14 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

