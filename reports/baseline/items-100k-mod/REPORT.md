# Baseline run: `items-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54892 -> 54892) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 13.073 / 17.846 / 83.977 / 107.964 |
| MSPT mean, ms | 16.244 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1338.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 2 | 6.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.99% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 4 | 4.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 5 | 4.00% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 1.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3567` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2089` |
| 9 | 1.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2135` |
| 10 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.37% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.92% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1484` |

