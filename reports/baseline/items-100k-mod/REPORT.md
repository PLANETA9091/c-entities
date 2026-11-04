# Baseline run: `items-100k-mod`

Server: `26.3-240-6b68358 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62905 -> 62905) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 28.155 / 83.681 / 152.750 / 227.932 |
| MSPT mean, ms | 39.259 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1605.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.21% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 5.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10094` |
| 3 | 4.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5978` |
| 4 | 4.65% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 3.99% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 6 | 3.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 2.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10047` |
| 9 | 1.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6049` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10050` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10087` |
| 12 | 1.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 14 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |

