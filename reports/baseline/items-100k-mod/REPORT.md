# Baseline run: `items-100k-mod`

Server: `26.3-239-13565bc (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62933 -> 62933) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 30.994 / 88.930 / 158.914 / 179.278 |
| MSPT mean, ms | 41.759 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1258.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.57% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 4.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10092` |
| 3 | 4.10% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6751` |
| 5 | 3.63% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 6 | 2.99% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10085` |
| 7 | 2.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5978` |
| 8 | 2.62% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 9 | 2.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 2.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 11 | 1.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 12 | 1.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10048` |
| 13 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 14 | 1.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6082` |

