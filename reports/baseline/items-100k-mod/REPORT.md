# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62895 -> 62895) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 27.972 / 79.099 / 169.639 / 251.951 |
| MSPT mean, ms | 39.520 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1501.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.79% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 2 | 9.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5473` |
| 3 | 6.85% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.79% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.34% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4054` |
| 6 | 2.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 1.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3927` |
| 8 | 1.47% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9407` |
| 10 | 1.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5478` |
| 11 | 1.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5549` |
| 12 | 1.19% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.01% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2760` |
| 15 | 0.95% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |

