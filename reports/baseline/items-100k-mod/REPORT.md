# Baseline run: `items-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62999 -> 62999) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 39.113 / 59.908 / 127.155 / 307.789 |
| MSPT mean, ms | 47.285 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1414.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.98% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 5.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7120` |
| 3 | 4.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10983` |
| 4 | 3.65% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6357` |
| 7 | 2.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 2.61% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4777` |
| 10 | 2.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 2.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7130` |
| 12 | 2.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.61% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10833` |
| 14 | 1.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6284` |
| 15 | 1.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |

