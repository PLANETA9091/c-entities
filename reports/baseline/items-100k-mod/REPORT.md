# Baseline run: `items-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62792 -> 62792) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 33.636 / 85.648 / 142.103 / 213.452 |
| MSPT mean, ms | 44.322 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1544.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7872` |
| 2 | 10.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4214` |
| 3 | 5.03% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.99% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7819` |
| 5 | 4.82% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 4.09% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 2.72% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2909` |
| 8 | 2.60% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2801` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4932` |
| 11 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.08% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4220` |
| 14 | 0.98% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4251` |

