# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62910 -> 62910) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 17.531 / 25.490 / 101.543 / 134.032 |
| MSPT mean, ms | 23.208 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1281.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 2 | 8.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 3 | 6.36% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 5.13% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 5 | 2.30% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2463` |
| 7 | 2.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3895` |
| 8 | 2.07% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2417` |
| 13 | 0.88% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 15 | 0.82% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |

