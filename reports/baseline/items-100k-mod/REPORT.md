# Baseline run: `items-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62973 -> 62972) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.425 / 58.615 / 135.624 / 228.431 |
| MSPT mean, ms | 40.929 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1628.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.38% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 4.55% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5141` |
| 3 | 3.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7770` |
| 4 | 3.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.47% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6767` |
| 8 | 2.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.11% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12242` |
| 10 | 1.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12392` |
| 11 | 1.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7008` |
| 12 | 1.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6955` |
| 13 | 1.42% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 1.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12377` |
| 15 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

