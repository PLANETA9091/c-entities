# Baseline run: `items-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62885 -> 62885) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.990 / 52.942 / 116.961 / 381.401 |
| MSPT mean, ms | 37.349 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1784.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8307` |
| 2 | 4.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5775` |
| 3 | 4.57% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 5 | 4.29% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5771` |
| 7 | 2.80% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8108` |
| 9 | 2.55% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13922` |
| 10 | 2.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8086` |
| 11 | 1.60% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5777` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7970` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8104` |
| 15 | 1.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14344` |

