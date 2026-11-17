# Baseline run: `mobs-25k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24561 -> 24561) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 313.217 / 1612.316 / 2737.118 / 3010.006 |
| MSPT mean, ms | 617.025 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 2489.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 19.05% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1842` |
| 2 | 3.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3686` |
| 3 | 3.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3685` |
| 4 | 1.99% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 5 | 1.54% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 6 | 1.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3406` |
| 7 | 1.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3753` |
| 9 | 1.25% | `java.util.ArrayList.add(Object) line: 497` |
| 10 | 1.16% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 11 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.00% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 0.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.82% | `java.util.ArrayList.elementData(int) line: 412` |

