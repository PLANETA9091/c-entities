# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62992 -> 62992) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 28.817 / 72.150 / 146.644 / 193.582 |
| MSPT mean, ms | 39.319 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1236.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5546` |
| 2 | 5.72% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 3 | 5.24% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 3.98% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9625` |
| 6 | 3.25% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9632` |
| 7 | 3.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 1.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 9 | 1.70% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.66% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9585` |
| 11 | 1.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9588` |
| 12 | 1.52% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5555` |
| 14 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |

