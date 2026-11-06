# Baseline run: `items-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62961 -> 62961) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.815 / 95.152 / 167.780 / 191.222 |
| MSPT mean, ms | 44.528 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1430.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10508` |
| 2 | 8.54% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 7.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6120` |
| 4 | 5.27% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 4.12% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4568` |
| 6 | 2.11% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10499` |
| 7 | 2.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 1.90% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6191` |
| 11 | 1.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4436` |
| 12 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3199` |
| 14 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3237` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10503` |

