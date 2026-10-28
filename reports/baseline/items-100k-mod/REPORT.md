# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63011 -> 63011) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.132 / 87.211 / 153.158 / 392.269 |
| MSPT mean, ms | 40.330 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1185.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7635` |
| 2 | 7.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3986` |
| 3 | 4.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 4.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.59% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 6 | 3.49% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2682` |
| 7 | 3.28% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7582` |
| 8 | 3.02% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2584` |
| 10 | 1.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4695` |
| 11 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3992` |
| 12 | 1.03% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4023` |
| 15 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |

