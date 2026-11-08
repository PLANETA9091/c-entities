# Baseline run: `items-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63027 -> 63027) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 28.011 / 43.828 / 109.377 / 186.797 |
| MSPT mean, ms | 35.603 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1429.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.59% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 7.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 3 | 6.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 4 | 4.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.64% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4729` |
| 7 | 3.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6309` |
| 8 | 3.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.07% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 2.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4597` |
| 11 | 1.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6380` |
| 12 | 1.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10785` |
| 14 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6236` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4677` |

