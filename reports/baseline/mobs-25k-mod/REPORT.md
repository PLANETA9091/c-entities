# Baseline run: `mobs-25k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24742 -> 24742) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 369.816 / 2085.304 / 3079.285 / 3392.747 |
| MSPT mean, ms | 776.464 |
| TPS estimate (1000/p50) | 2.7 |
| heap used / max, MB | 2171.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.01% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1836` |
| 2 | 4.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3680` |
| 3 | 3.71% | `java.util.ArrayList.elementData(int) line: 412` |
| 4 | 3.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3679` |
| 5 | 2.74% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 6 | 2.18% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 7 | 1.67% | `java.util.ArrayList.add(Object) line: 497` |
| 8 | 1.52% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 9 | 1.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3747` |
| 10 | 1.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3400` |
| 11 | 1.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.11% | `java.util.ArrayList.grow() line: 245` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1837` |
| 14 | 0.96% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14355` |

