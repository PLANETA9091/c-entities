# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62989 -> 62989) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 36.573 / 85.798 / 149.227 / 206.117 |
| MSPT mean, ms | 46.703 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1183.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9269` |
| 2 | 10.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5427` |
| 3 | 5.91% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.99% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4020` |
| 6 | 3.93% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 7 | 3.84% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9146` |
| 8 | 1.78% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3893` |
| 11 | 0.91% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9228` |
| 12 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2742` |
| 15 | 0.68% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

