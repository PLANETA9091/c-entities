# Baseline run: `items-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62952 -> 62952) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 31.932 / 49.060 / 121.479 / 242.174 |
| MSPT mean, ms | 39.930 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1450.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 2 | 6.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 3 | 6.34% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 5.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6309` |
| 5 | 4.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 4.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.61% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 3.35% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4729` |
| 9 | 1.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.68% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4597` |
| 13 | 1.50% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10785` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10797` |
| 15 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6236` |

