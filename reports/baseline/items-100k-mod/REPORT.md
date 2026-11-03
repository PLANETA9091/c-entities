# Baseline run: `items-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62964 -> 62964) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 31.343 / 76.257 / 136.588 / 203.992 |
| MSPT mean, ms | 41.279 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1423.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.94% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 4.19% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 4.15% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10059` |
| 4 | 3.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6746` |
| 6 | 2.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 2.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5973` |
| 8 | 2.82% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10052` |
| 9 | 2.80% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 10 | 1.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5947` |
| 11 | 1.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.56% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 14 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6044` |
| 15 | 1.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10012` |

