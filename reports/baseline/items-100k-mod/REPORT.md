# Baseline run: `items-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54783 -> 54783) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 15.983 / 23.873 / 102.547 / 137.490 |
| MSPT mean, ms | 20.601 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1293.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 2 | 10.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 3 | 5.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 5 | 3.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2381` |
| 6 | 2.67% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 2.16% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 2.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3813` |
| 10 | 1.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2335` |
| 11 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.04% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.02% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7413` |
| 15 | 0.83% | `net.minecraft.world.entity.Entity.getId() line: 651` |

