# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62866 -> 62866) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 38.820 / 80.887 / 135.270 / 168.371 |
| MSPT mean, ms | 48.748 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1489.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4719` |
| 2 | 8.74% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8421` |
| 3 | 5.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8474` |
| 4 | 3.87% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 3.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5512` |
| 6 | 2.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.87% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 8 | 2.75% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3420` |
| 9 | 2.40% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 1.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5510` |
| 13 | 1.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2448` |
| 14 | 1.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 15 | 1.12% | `net.minecraft.world.entity.Entity.getId() line: 651` |

