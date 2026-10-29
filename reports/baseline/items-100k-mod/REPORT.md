# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62966 -> 62966) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.181 / 87.669 / 162.324 / 330.043 |
| MSPT mean, ms | 43.801 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1307.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7826` |
| 2 | 9.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4168` |
| 3 | 5.40% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.09% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2863` |
| 7 | 2.98% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 2.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7773` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2765` |
| 10 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.20% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4886` |
| 14 | 0.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4174` |
| 15 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

