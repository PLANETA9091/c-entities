# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62896 -> 62896) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.793 / 80.037 / 156.116 / 213.206 |
| MSPT mean, ms | 45.453 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1458.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4726` |
| 2 | 11.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8481` |
| 3 | 6.17% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.47% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.37% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 6 | 3.04% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3427` |
| 7 | 2.97% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8428` |
| 8 | 1.70% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3319` |
| 11 | 0.87% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2439` |
| 13 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5517` |
| 15 | 0.82% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8442` |

