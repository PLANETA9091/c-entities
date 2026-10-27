# Baseline run: `items-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54809 -> 54809) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 16.106 / 20.987 / 98.692 / 150.551 |
| MSPT mean, ms | 18.369 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1139.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7176` |
| 2 | 6.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4236` |
| 3 | 5.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 5.61% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7123` |
| 5 | 3.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2114` |
| 6 | 2.90% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3527` |
| 8 | 1.82% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7137` |
| 11 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 0.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 14 | 0.79% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.75% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2252` |

