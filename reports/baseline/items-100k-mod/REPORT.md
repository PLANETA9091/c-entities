# Baseline run: `items-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62982 -> 62982) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 27.999 / 70.246 / 160.378 / 215.649 |
| MSPT mean, ms | 38.504 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1604.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.85% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 5.62% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 5.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 4 | 4.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9777` |
| 5 | 3.79% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 6 | 3.04% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 1.83% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9770` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9730` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9733` |
| 11 | 1.19% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5781` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5684` |
| 14 | 1.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 15 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

