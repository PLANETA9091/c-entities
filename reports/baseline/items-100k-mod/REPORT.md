# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62985 -> 62985) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 33.979 / 90.046 / 141.061 / 203.796 |
| MSPT mean, ms | 44.978 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1386.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.26% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8229` |
| 2 | 7.03% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 3 | 5.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8282` |
| 4 | 5.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4594` |
| 5 | 3.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5320` |
| 6 | 3.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.47% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 2.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3181` |
| 9 | 2.14% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2339` |
| 10 | 1.95% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 11 | 1.90% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3289` |
| 12 | 1.77% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 13 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

