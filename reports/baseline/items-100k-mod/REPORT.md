# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62941 -> 62941) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.683 / 96.185 / 160.198 / 207.341 |
| MSPT mean, ms | 45.893 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1238.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 17.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5435` |
| 3 | 4.26% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 3.84% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4028` |
| 5 | 3.21% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 6 | 2.13% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9322` |
| 7 | 1.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3901` |
| 8 | 1.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9317` |
| 9 | 1.33% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5441` |
| 11 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.18% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.08% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9316` |

