# Baseline run: `items-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63059 -> 63059) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 21.181 / 31.798 / 91.240 / 153.937 |
| MSPT mean, ms | 26.934 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1834.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.17% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 6.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4627` |
| 3 | 5.19% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4759` |
| 4 | 4.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10965` |
| 5 | 3.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6266` |
| 7 | 2.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.57% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 9 | 2.23% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4707` |
| 10 | 2.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7112` |
| 12 | 1.64% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7102` |
| 14 | 1.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6410` |

