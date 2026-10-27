# Baseline run: `items-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54671 -> 54671) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 10.013 / 13.341 / 73.560 / 107.014 |
| MSPT mean, ms | 11.813 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1237.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7145` |
| 2 | 4.66% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7092` |
| 3 | 3.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.28% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 5 | 3.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 6 | 2.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 2.17% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 8 | 2.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2037` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3496` |
| 10 | 1.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2083` |
| 12 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.00% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 15 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1433` |

