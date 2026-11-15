# Baseline run: `mixed-150k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113987 -> 113961) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 95.787 / 165.776 / 286.345 / 371.573 |
| MSPT mean, ms | 114.092 |
| TPS estimate (1000/p50) | 10.4 |
| heap used / max, MB | 2065.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8321` |
| 2 | 8.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7569` |
| 3 | 5.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13127` |
| 4 | 4.44% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.09% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 9 | 1.72% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 12 | 1.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13026` |
| 13 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7440` |
| 14 | 1.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 15 | 1.01% | `net.minecraft.world.entity.Entity.getType() line: 633` |

