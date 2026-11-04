# Baseline run: `items-100k-mod`

Server: `26.3-241-5050b0f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62873 -> 62873) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.602 / 95.233 / 159.166 / 257.392 |
| MSPT mean, ms | 44.415 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1345.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10175` |
| 2 | 7.39% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 6.20% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 5.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 5 | 3.72% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 6 | 3.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 1.97% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10166` |
| 9 | 1.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6092` |
| 10 | 1.67% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 12 | 0.98% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10170` |
| 13 | 0.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.88% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |

