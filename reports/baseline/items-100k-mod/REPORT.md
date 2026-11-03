# Baseline run: `items-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62898 -> 62898) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 19.892 / 48.588 / 112.434 / 186.940 |
| MSPT mean, ms | 28.054 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1461.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.24% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 3.77% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 3.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 4 | 3.31% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4152` |
| 5 | 3.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 6 | 3.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 7 | 2.82% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9777` |
| 8 | 2.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2835` |
| 9 | 1.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.81% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 12 | 1.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2873` |
| 13 | 1.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5684` |
| 14 | 1.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 15 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5781` |

