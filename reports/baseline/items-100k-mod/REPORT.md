# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63015 -> 63015) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.155 / 95.625 / 164.875 / 211.414 |
| MSPT mean, ms | 46.260 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1402.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 10.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 6.05% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.96% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 4.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 4.74% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 7 | 3.70% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 8 | 2.25% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 11 | 0.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 12 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 14 | 0.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 15 | 0.83% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |

