# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62879 -> 62879) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 33.809 / 91.674 / 150.126 / 225.567 |
| MSPT mean, ms | 45.275 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1386.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8282` |
| 2 | 10.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4594` |
| 3 | 5.51% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 4.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8229` |
| 7 | 2.76% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3289` |
| 8 | 2.12% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5318` |
| 10 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.03% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4600` |
| 13 | 0.98% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3181` |
| 15 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

