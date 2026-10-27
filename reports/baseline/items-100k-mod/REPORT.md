# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54934 -> 54934) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 11.265 / 15.842 / 76.251 / 103.896 |
| MSPT mean, ms | 14.237 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1291.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 2 | 6.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 3 | 4.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 4 | 4.84% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 5 | 4.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 2.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2381` |
| 8 | 2.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2335` |
| 9 | 2.04% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 10 | 1.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3813` |
| 12 | 1.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.18% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 14 | 1.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |

