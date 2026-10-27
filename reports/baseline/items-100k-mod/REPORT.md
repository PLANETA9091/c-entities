# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54826 -> 54826) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 14.277 / 21.000 / 93.338 / 120.442 |
| MSPT mean, ms | 19.346 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1363.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7511` |
| 2 | 9.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4571` |
| 3 | 6.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7458` |
| 5 | 2.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 6 | 2.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3862` |
| 7 | 2.01% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 2.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2430` |
| 9 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.31% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 11 | 1.17% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 12 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 0.98% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7472` |
| 14 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1764` |
| 15 | 0.75% | `net.minecraft.world.entity.Entity.getId() line: 651` |

