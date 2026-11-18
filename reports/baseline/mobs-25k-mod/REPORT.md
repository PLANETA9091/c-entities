# Baseline run: `mobs-25k-mod`

Server: `26.3-255-c22378f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24564 -> 24564) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 417.495 / 2731.550 / 2870.113 / 3168.453 |
| MSPT mean, ms | 1087.650 |
| TPS estimate (1000/p50) | 2.4 |
| heap used / max, MB | 2190.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 27.92% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1894` |
| 2 | 6.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3749` |
| 3 | 3.94% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3748` |
| 4 | 2.43% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 5 | 2.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3469` |
| 6 | 1.93% | `java.util.ArrayList.add(Object) line: 497` |
| 7 | 1.81% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 8 | 1.72% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3816` |
| 9 | 1.60% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 10 | 1.27% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1895` |
| 11 | 1.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.01% | `java.util.ArrayList.elementData(int) line: 412` |
| 13 | 0.93% | `java.util.ArrayList.add(Object) line: 496` |
| 14 | 0.87% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 15 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

