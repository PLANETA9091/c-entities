# Baseline run: `items-100k-mod`

Server: `26.3-241-5050b0f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62982 -> 62982) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.293 / 81.972 / 166.739 / 237.475 |
| MSPT mean, ms | 40.624 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1397.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.91% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10100` |
| 2 | 6.25% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 4.41% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 3.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5979` |
| 5 | 3.74% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4468` |
| 6 | 3.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.13% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10091` |
| 8 | 1.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4336` |
| 10 | 1.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6050` |
| 11 | 1.25% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10095` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5953` |
| 13 | 1.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.11% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5944` |

