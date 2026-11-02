# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62917 -> 62917) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 21.304 / 62.067 / 113.604 / 176.534 |
| MSPT mean, ms | 28.992 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1277.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 2 | 5.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9118` |
| 3 | 3.58% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9204` |
| 5 | 3.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6173` |
| 6 | 2.99% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 2.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5424` |
| 8 | 2.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3890` |
| 9 | 2.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2701` |
| 10 | 1.75% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3970` |
| 11 | 1.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.63% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4017` |
| 13 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5462` |
| 15 | 1.10% | `net.minecraft.world.entity.Entity.level() line: 5421` |

