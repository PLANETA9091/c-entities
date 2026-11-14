# Baseline run: `items-100k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63027 -> 63026) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.698 / 49.211 / 116.253 / 212.515 |
| MSPT mean, ms | 37.817 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1716.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 17.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7465` |
| 2 | 5.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5199` |
| 3 | 4.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8218` |
| 5 | 3.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.64% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13024` |
| 8 | 2.77% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5203` |
| 10 | 1.94% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12738` |
| 11 | 1.81% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7405` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7361` |
| 15 | 1.09% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |

