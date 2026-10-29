# Baseline run: `items-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62913 -> 62913) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.131 / 90.310 / 138.300 / 251.159 |
| MSPT mean, ms | 44.091 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1375.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8130` |
| 2 | 10.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4448` |
| 3 | 6.45% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.10% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8077` |
| 7 | 2.92% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3143` |
| 8 | 2.51% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3035` |
| 11 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.94% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4454` |
| 14 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4485` |
| 15 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

