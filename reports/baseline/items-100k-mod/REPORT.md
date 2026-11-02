# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62875 -> 62875) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.881 / 85.823 / 138.639 / 261.874 |
| MSPT mean, ms | 40.348 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1191.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 16.12% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5435` |
| 3 | 5.17% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 3.55% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4028` |
| 5 | 2.83% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 6 | 2.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9335` |
| 7 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3901` |
| 8 | 1.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9330` |
| 9 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.22% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.01% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9329` |
| 13 | 0.93% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6213` |

