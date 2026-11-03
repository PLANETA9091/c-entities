# Baseline run: `items-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62978 -> 62978) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.969 / 85.841 / 179.585 / 232.289 |
| MSPT mean, ms | 41.803 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1287.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.15% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 6.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 3 | 5.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9777` |
| 4 | 4.51% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 3.95% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 6 | 2.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9770` |
| 7 | 2.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.79% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9733` |
| 10 | 1.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9730` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 12 | 1.38% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5781` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 15 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

