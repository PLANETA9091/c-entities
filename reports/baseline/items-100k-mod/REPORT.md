# Baseline run: `items-100k-mod`

Server: `26.3-235-bb10cf0 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62906 -> 62906) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.665 / 87.898 / 161.376 / 207.240 |
| MSPT mean, ms | 45.450 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1487.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.12% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 2 | 6.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 3 | 5.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 4 | 5.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5145` |
| 5 | 4.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5874` |
| 6 | 3.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.58% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 8 | 1.98% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 9 | 1.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 11 | 1.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.58% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.56% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 14 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

