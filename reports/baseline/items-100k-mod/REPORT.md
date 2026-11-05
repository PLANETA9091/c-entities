# Baseline run: `items-100k-mod`

Server: `26.3-242-fd28d65 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62961 -> 62961) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.456 / 83.420 / 168.523 / 202.477 |
| MSPT mean, ms | 43.249 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1581.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10314` |
| 2 | 7.75% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 5.58% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 5.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 5 | 3.76% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 6 | 2.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.07% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10305` |
| 8 | 2.05% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 2.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 11 | 1.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6092` |
| 12 | 1.07% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5965` |
| 15 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |

