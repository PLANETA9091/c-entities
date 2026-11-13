# Baseline run: `items-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 50669 -> 50669) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 24.247 / 37.050 / 100.944 / 245.851 |
| MSPT mean, ms | 30.369 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1644.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.85% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 5.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11978` |
| 3 | 4.98% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4887` |
| 4 | 4.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7347` |
| 5 | 3.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6464` |
| 9 | 1.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6286` |
| 10 | 1.77% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.36% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11828` |
| 12 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6290` |
| 13 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.91% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 0.89% | `net.minecraft.world.entity.Entity.getX() line: 5100` |

