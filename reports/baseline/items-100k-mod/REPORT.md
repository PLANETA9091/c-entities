# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62946 -> 62946) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 43.974 / 105.583 / 195.484 / 250.682 |
| MSPT mean, ms | 56.476 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1260.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9193` |
| 2 | 11.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5416` |
| 3 | 7.75% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 5 | 4.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4009` |
| 6 | 4.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 4.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9107` |
| 8 | 1.95% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.27% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9152` |
| 11 | 0.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6152` |
| 12 | 0.84% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5453` |

