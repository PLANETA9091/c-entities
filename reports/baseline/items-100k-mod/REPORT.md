# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62883 -> 62883) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.836 / 79.440 / 145.839 / 235.174 |
| MSPT mean, ms | 43.856 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1594.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4803` |
| 2 | 11.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8558` |
| 3 | 4.97% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.09% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8505` |
| 6 | 3.05% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 7 | 2.74% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3504` |
| 8 | 2.66% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4867` |
| 10 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2516` |
| 12 | 1.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5594` |
| 13 | 1.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3396` |
| 14 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.89% | `net.minecraft.world.entity.Entity.getId() line: 651` |

