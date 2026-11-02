# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62936 -> 62936) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 25.653 / 64.334 / 126.131 / 161.858 |
| MSPT mean, ms | 35.377 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1567.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 8.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5453` |
| 3 | 5.60% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4033` |
| 4 | 4.65% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 4.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9354` |
| 6 | 3.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 2.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3906` |
| 8 | 2.17% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 2.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 2.00% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.77% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3986` |
| 12 | 1.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6232` |
| 13 | 1.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2712` |
| 14 | 1.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6222` |
| 15 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5459` |

