# Baseline run: `items-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62956 -> 62956) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 25.322 / 40.840 / 101.760 / 122.683 |
| MSPT mean, ms | 32.356 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1472.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.50% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 5.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10983` |
| 3 | 4.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7120` |
| 4 | 4.20% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.15% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4777` |
| 6 | 4.07% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 3.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6284` |
| 9 | 2.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4645` |
| 10 | 1.82% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.79% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4725` |
| 12 | 1.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7130` |
| 13 | 1.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 1.09% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10833` |

