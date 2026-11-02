# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63035 -> 63035) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.811 / 85.073 / 139.553 / 232.131 |
| MSPT mean, ms | 45.267 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1387.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.89% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |
| 2 | 8.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5413` |
| 3 | 5.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9126` |
| 4 | 5.70% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 5 | 3.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6159` |
| 6 | 3.02% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.53% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 8 | 2.46% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 2.43% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4006` |
| 10 | 2.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3879` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 13 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.14% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

