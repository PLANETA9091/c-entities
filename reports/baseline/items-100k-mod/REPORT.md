# Baseline run: `items-100k-mod`

Server: `26.3-242-fd28d65 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62883 -> 62883) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.980 / 88.991 / 149.381 / 206.865 |
| MSPT mean, ms | 40.397 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1575.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10280` |
| 2 | 7.58% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 5.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 4.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 5 | 3.54% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 6 | 2.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10271` |
| 8 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.68% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6092` |
| 11 | 1.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 12 | 1.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 14 | 0.96% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10275` |

