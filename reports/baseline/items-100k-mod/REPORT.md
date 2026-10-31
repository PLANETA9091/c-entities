# Baseline run: `items-100k-mod`

Server: `26.3-235-bb10cf0 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63004 -> 63004) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 33.623 / 92.141 / 157.768 / 182.480 |
| MSPT mean, ms | 45.441 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1506.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 2 | 11.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 3 | 5.85% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.20% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.84% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 6 | 2.96% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 7 | 2.78% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 8 | 2.19% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 10 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5097` |
| 12 | 0.96% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2661` |
| 13 | 0.93% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.85% | `net.minecraft.world.entity.Entity.getId() line: 651` |

