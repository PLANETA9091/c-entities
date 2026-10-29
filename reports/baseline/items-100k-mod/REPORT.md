# Baseline run: `items-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62904 -> 62904) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.407 / 96.023 / 159.172 / 240.920 |
| MSPT mean, ms | 46.380 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1367.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7984` |
| 2 | 10.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4302` |
| 3 | 7.22% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.69% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 4.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7931` |
| 7 | 3.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2997` |
| 8 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5020` |
| 10 | 1.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2889` |
| 11 | 0.99% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 12 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.96% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7945` |
| 14 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.78% | `net.minecraft.world.entity.Entity.getId() line: 651` |

