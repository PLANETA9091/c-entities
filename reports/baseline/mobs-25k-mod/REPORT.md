# Baseline run: `mobs-25k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25064 -> 25064) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 433.201 / 644.210 / 777.897 / 816.931 |
| MSPT mean, ms | 466.306 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 1989.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.90% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 2 | 2.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2831` |
| 3 | 2.56% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.00% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 5 | 1.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.60% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 9 | 1.49% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5094` |
| 11 | 1.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10806` |
| 12 | 1.32% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5031` |

