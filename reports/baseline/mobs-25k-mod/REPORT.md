# Baseline run: `mobs-25k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24677 -> 24677) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 2712.236 / 2875.846 / 3025.522 / 3114.843 |
| MSPT mean, ms | 2729.314 |
| TPS estimate (1000/p50) | 0.4 |
| heap used / max, MB | 1518.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.81% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 2 | 8.37% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2895` |
| 3 | 7.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2894` |
| 4 | 4.70% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 5 | 3.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2658` |
| 6 | 3.24% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 7 | 2.83% | `java.util.ArrayList.add(Object) line: 497` |
| 8 | 2.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2962` |
| 9 | 2.05% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 10 | 1.84% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 11 | 1.59% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1378` |
| 12 | 1.45% | `java.util.ArrayList.elementData(int) line: 412` |
| 13 | 1.14% | `java.util.ArrayList.add(Object) line: 496` |
| 14 | 1.12% | `java.util.ArrayList.grow() line: 245` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2877` |

