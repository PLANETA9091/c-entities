# Baseline run: `items-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63048 -> 63049) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 30.596 / 53.458 / 127.553 / 372.887 |
| MSPT mean, ms | 38.362 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1626.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8102` |
| 2 | 6.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8945` |
| 3 | 6.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7881` |
| 4 | 4.99% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.29% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.85% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5570` |
| 7 | 3.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5566` |
| 8 | 2.98% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 2.69% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13717` |
| 10 | 1.99% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 11 | 1.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14139` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5572` |
| 13 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7899` |
| 14 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7765` |
| 15 | 0.99% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13865` |

