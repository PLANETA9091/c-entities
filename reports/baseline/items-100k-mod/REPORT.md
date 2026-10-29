# Baseline run: `items-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62868 -> 62868) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.302 / 88.477 / 163.397 / 181.745 |
| MSPT mean, ms | 46.031 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1417.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8068` |
| 2 | 9.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4386` |
| 3 | 6.55% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.18% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.61% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.62% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8015` |
| 7 | 3.27% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3081` |
| 8 | 2.43% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.17% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5104` |
| 12 | 0.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8029` |
| 13 | 0.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2973` |
| 14 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

