# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62905 -> 62905) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 35.412 / 88.988 / 158.461 / 215.893 |
| MSPT mean, ms | 46.437 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1553.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7819` |
| 2 | 10.69% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4161` |
| 3 | 6.31% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.82% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.43% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2856` |
| 7 | 3.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7766` |
| 8 | 1.73% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4879` |
| 11 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4167` |
| 12 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7780` |
| 13 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2758` |
| 15 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

