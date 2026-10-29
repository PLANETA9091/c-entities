# Baseline run: `items-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62887 -> 62887) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.735 / 93.500 / 166.889 / 208.557 |
| MSPT mean, ms | 46.987 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1148.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8242` |
| 2 | 10.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4560` |
| 3 | 6.28% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.03% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3255` |
| 7 | 2.79% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8189` |
| 8 | 1.97% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5278` |
| 10 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3147` |
| 13 | 0.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8203` |
| 14 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2336` |
| 15 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4566` |

