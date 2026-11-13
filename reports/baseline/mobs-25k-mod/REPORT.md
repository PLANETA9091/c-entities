# Baseline run: `mobs-25k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24804 -> 24804) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 3534.786 / 3804.843 / 4088.265 / 4309.439 |
| MSPT mean, ms | 3575.539 |
| TPS estimate (1000/p50) | 0.3 |
| heap used / max, MB | 2001.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 44.82% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 2 | 7.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2894` |
| 3 | 5.04% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 4 | 4.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2895` |
| 5 | 3.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2658` |
| 6 | 2.12% | `java.util.ArrayList.elementData(int) line: 412` |
| 7 | 1.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2699` |
| 8 | 1.40% | `java.util.ArrayList.add(Object) line: 497` |
| 9 | 1.25% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1376` |
| 10 | 1.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2885` |
| 11 | 0.98% | `java.util.ArrayList.add(Object) line: 496` |
| 12 | 0.90% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1378` |
| 13 | 0.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2927` |
| 14 | 0.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2893` |
| 15 | 0.63% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |

