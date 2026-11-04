# Baseline run: `items-100k-mod`

Server: `26.3-241-5050b0f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62940 -> 62940) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 31.652 / 92.817 / 179.966 / 223.148 |
| MSPT mean, ms | 42.935 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1618.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.79% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10280` |
| 2 | 7.53% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 5.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 4 | 5.54% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 4.17% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 6 | 2.57% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 1.99% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10271` |
| 11 | 1.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6092` |
| 12 | 1.41% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10275` |
| 15 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

