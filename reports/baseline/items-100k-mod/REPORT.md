# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62911 -> 62911) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.468 / 82.093 / 144.273 / 174.796 |
| MSPT mean, ms | 39.681 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1120.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 7.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 5.42% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 4.49% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 6 | 4.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.34% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 8 | 2.76% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 10 | 1.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.50% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 13 | 0.99% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 14 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 15 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

