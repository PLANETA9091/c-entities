# Baseline run: `items-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54975 -> 54975) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 13.541 / 18.696 / 86.652 / 127.727 |
| MSPT mean, ms | 17.986 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1154.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 2 | 8.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 3 | 5.55% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.91% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 5 | 2.32% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2135` |
| 7 | 2.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 1.91% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3567` |
| 10 | 1.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 0.97% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7177` |
| 13 | 0.82% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2089` |
| 15 | 0.68% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |

