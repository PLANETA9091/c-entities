# Baseline run: `mobs-25k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24395 -> 24395) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 306.681 / 1525.957 / 2361.242 / 2784.365 |
| MSPT mean, ms | 582.836 |
| TPS estimate (1000/p50) | 3.3 |
| heap used / max, MB | 1681.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 18.36% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1786` |
| 2 | 3.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3572` |
| 3 | 2.37% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 4 | 1.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.05% | `java.util.ArrayList.elementData(int) line: 412` |
| 8 | 1.01% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3293` |
| 9 | 0.99% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 0.98% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 11 | 0.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 0.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 13 | 0.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3573` |
| 14 | 0.88% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3555` |

