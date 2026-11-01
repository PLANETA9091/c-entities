# Baseline run: `items-100k-mod`

Server: `26.3-235-bb10cf0 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62972 -> 62972) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 35.102 / 90.333 / 149.325 / 193.787 |
| MSPT mean, ms | 45.917 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1304.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 2 | 9.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5145` |
| 3 | 5.53% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 3.59% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 7 | 3.13% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 8 | 1.66% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 11 | 1.15% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 13 | 0.99% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 14 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8797` |

