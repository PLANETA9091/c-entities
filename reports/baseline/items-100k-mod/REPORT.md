# Baseline run: `items-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62921 -> 62921) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.135 / 91.951 / 169.987 / 242.076 |
| MSPT mean, ms | 42.850 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1380.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.66% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 5.46% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 4.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9655` |
| 4 | 4.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5588` |
| 5 | 4.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4082` |
| 6 | 3.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9648` |
| 8 | 2.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 9 | 2.03% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9611` |
| 10 | 2.03% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9608` |
| 11 | 1.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5659` |
| 12 | 1.86% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5553` |

