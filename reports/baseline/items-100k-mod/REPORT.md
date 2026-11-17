# Baseline run: `items-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62998 -> 62999) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 17.449 / 30.695 / 103.060 / 212.093 |
| MSPT mean, ms | 24.319 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1848.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 7.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9373` |
| 3 | 6.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8530` |
| 4 | 5.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.91% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7567` |
| 6 | 2.49% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7571` |
| 7 | 1.71% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 987` |
| 8 | 1.55% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 1.32% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13651` |
| 10 | 1.20% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7573` |
| 11 | 1.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5860` |
| 12 | 1.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5864` |
| 14 | 0.92% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7585` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5866` |

