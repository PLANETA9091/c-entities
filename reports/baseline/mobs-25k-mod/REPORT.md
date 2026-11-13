# Baseline run: `mobs-25k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24826 -> 24826) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 3308.742 / 3532.545 / 3745.549 / 3957.857 |
| MSPT mean, ms | 3326.687 |
| TPS estimate (1000/p50) | 0.3 |
| heap used / max, MB | 1645.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.14% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 2 | 7.91% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2882` |
| 3 | 4.84% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2881` |
| 4 | 4.65% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 5 | 4.17% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 6 | 3.79% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2949` |
| 7 | 3.63% | `java.util.ArrayList.add(Object) line: 497` |
| 8 | 2.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2645` |
| 9 | 1.98% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 10 | 1.89% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1378` |
| 11 | 1.59% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 12 | 1.51% | `java.util.ArrayList.add(Object) line: 496` |
| 13 | 1.50% | `java.util.ArrayList.elementData(int) line: 412` |
| 14 | 1.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2864` |
| 15 | 1.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2896` |

