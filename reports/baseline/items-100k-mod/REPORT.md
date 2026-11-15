# Baseline run: `items-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62974 -> 62974) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 27.068 / 44.634 / 110.153 / 210.669 |
| MSPT mean, ms | 34.015 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1672.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7983` |
| 2 | 6.17% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8794` |
| 4 | 4.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5458` |
| 6 | 3.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5454` |
| 7 | 3.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7791` |
| 8 | 2.82% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.87% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.82% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13566` |
| 11 | 1.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7653` |
| 12 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7787` |
| 13 | 1.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5460` |
| 14 | 1.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7769` |
| 15 | 1.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13988` |

