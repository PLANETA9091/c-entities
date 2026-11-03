# Baseline run: `items-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62899 -> 62899) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.766 / 76.803 / 175.406 / 223.314 |
| MSPT mean, ms | 41.576 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1437.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.55% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 6.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5584` |
| 3 | 5.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9651` |
| 4 | 4.81% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 5 | 4.43% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 6 | 2.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 2.10% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9644` |
| 9 | 1.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9604` |
| 10 | 1.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5655` |
| 11 | 1.59% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9607` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5549` |
| 13 | 1.04% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 14 | 1.02% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

