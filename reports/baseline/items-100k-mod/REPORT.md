# Baseline run: `items-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54829 -> 54829) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 8.975 / 12.571 / 41.475 / 103.109 |
| MSPT mean, ms | 10.676 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1285.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7195` |
| 2 | 6.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7142` |
| 3 | 6.11% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 3.51% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 3.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2087` |
| 7 | 3.18% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 8 | 2.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2133` |
| 9 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4255` |
| 11 | 1.63% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 12 | 1.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2162` |
| 13 | 1.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1482` |

