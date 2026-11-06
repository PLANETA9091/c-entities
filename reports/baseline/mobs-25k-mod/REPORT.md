# Baseline run: `mobs-25k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25043 -> 25043) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 262.162 / 459.315 / 578.143 / 634.137 |
| MSPT mean, ms | 300.507 |
| TPS estimate (1000/p50) | 3.8 |
| heap used / max, MB | 1864.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 4.73% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 3 | 4.60% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 4 | 2.01% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.53% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.45% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.09% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 10 | 1.09% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 11 | 1.05% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10392` |
| 12 | 1.01% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 13 | 0.97% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 0.94% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1292` |

