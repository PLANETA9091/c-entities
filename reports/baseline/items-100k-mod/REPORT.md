# Baseline run: `items-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62866 -> 62866) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 36.716 / 85.697 / 158.523 / 240.056 |
| MSPT mean, ms | 47.425 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1334.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.14% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7989` |
| 2 | 7.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4360` |
| 3 | 6.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 5.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8042` |
| 5 | 3.81% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 6 | 3.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5080` |
| 7 | 3.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.87% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3055` |
| 9 | 2.83% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.01% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2947` |
| 15 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5078` |

