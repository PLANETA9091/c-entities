# Baseline run: `items-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62903 -> 62903) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 30.516 / 85.781 / 174.568 / 229.905 |
| MSPT mean, ms | 41.747 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1346.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.43% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 5.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5650` |
| 3 | 4.83% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 4.78% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9717` |
| 5 | 4.41% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4144` |
| 6 | 2.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9710` |
| 8 | 2.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9673` |
| 9 | 1.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.85% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9670` |
| 11 | 1.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5721` |
| 12 | 1.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4012` |
| 13 | 1.52% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 14 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.97% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |

