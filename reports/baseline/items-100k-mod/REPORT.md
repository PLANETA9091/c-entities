# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63003 -> 63003) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.457 / 91.479 / 149.043 / 285.113 |
| MSPT mean, ms | 45.890 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1087.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8964` |
| 2 | 10.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5251` |
| 3 | 6.16% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 3.79% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3844` |
| 7 | 3.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8911` |
| 8 | 1.96% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 0.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3717` |
| 11 | 0.93% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 12 | 0.91% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8925` |
| 13 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.88% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

