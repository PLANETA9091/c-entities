# Baseline run: `items-100k-mod`

Server: `26.3-241-5050b0f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62947 -> 62947) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 33.662 / 75.458 / 138.117 / 190.663 |
| MSPT mean, ms | 42.784 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1289.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.63% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10280` |
| 2 | 7.19% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 3.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 4 | 3.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.73% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 6 | 2.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10271` |
| 7 | 2.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 8 | 2.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 9 | 2.11% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 2.00% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 11 | 1.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 13 | 1.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5965` |
| 14 | 1.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.27% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |

