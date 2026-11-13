# Baseline run: `mobs-25k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24787 -> 24787) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 3511.266 / 3760.439 / 3928.370 / 4262.820 |
| MSPT mean, ms | 3540.030 |
| TPS estimate (1000/p50) | 0.3 |
| heap used / max, MB | 1975.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 43.69% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1378` |
| 2 | 7.88% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2895` |
| 3 | 4.71% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 4 | 4.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2896` |
| 5 | 3.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2659` |
| 6 | 1.98% | `java.util.ArrayList.elementData(int) line: 412` |
| 7 | 1.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2700` |
| 8 | 1.29% | `java.util.ArrayList.add(Object) line: 497` |
| 9 | 1.24% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 10 | 1.02% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 11 | 0.99% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1379` |
| 12 | 0.88% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2886` |
| 13 | 0.86% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2878` |
| 14 | 0.81% | `java.util.ArrayList.add(Object) line: 496` |
| 15 | 0.74% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2963` |

