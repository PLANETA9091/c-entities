# Baseline run: `items-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62996 -> 62996) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.576 / 45.849 / 109.591 / 198.992 |
| MSPT mean, ms | 37.176 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1606.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |
| 2 | 6.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10970` |
| 3 | 5.48% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 5.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 5.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6344` |
| 6 | 3.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 2.60% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4764` |
| 8 | 2.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.40% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6415` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4632` |
| 12 | 1.42% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10820` |
| 13 | 1.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

