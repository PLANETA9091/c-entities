# Baseline run: `items-100k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63042 -> 63042) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.142 / 54.227 / 127.399 / 268.074 |
| MSPT mean, ms | 40.121 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1731.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 17.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7547` |
| 2 | 5.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8299` |
| 3 | 4.90% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 4.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 4.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 7 | 2.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.48% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12819` |
| 9 | 2.08% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13105` |
| 11 | 1.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5234` |
| 12 | 0.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13081` |
| 13 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5236` |
| 14 | 0.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

