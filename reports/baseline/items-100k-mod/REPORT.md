# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62961 -> 62961) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 37.178 / 97.625 / 177.181 / 217.013 |
| MSPT mean, ms | 49.556 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1297.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.54% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9126` |
| 2 | 11.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5413` |
| 3 | 6.82% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 4.01% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4006` |
| 7 | 3.80% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |
| 8 | 1.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 9 | 1.45% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 10 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.04% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9087` |
| 13 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3879` |
| 15 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

