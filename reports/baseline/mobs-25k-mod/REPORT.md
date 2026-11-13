# Baseline run: `mobs-25k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24760 -> 24760) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 2946.696 / 3154.238 / 3303.571 / 3452.325 |
| MSPT mean, ms | 2947.153 |
| TPS estimate (1000/p50) | 0.3 |
| heap used / max, MB | 2201.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 44.87% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 2 | 8.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2881` |
| 3 | 4.33% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 4 | 3.89% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2882` |
| 5 | 2.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2645` |
| 6 | 1.85% | `java.util.ArrayList.elementData(int) line: 412` |
| 7 | 1.56% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1376` |
| 8 | 1.35% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 9 | 0.93% | `java.util.ArrayList.add(Object) line: 497` |
| 10 | 0.89% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 11 | 0.87% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1378` |
| 12 | 0.83% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 13 | 0.72% | `java.util.ArrayList.add(Object) line: 496` |
| 14 | 0.68% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2864` |
| 15 | 0.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2896` |

