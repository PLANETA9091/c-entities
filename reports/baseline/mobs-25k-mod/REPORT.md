# Baseline run: `mobs-25k-mod`

Server: `26.3-241-5050b0f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25068 -> 25068) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 291.813 / 492.409 / 608.381 / 696.184 |
| MSPT mean, ms | 330.225 |
| TPS estimate (1000/p50) | 3.4 |
| heap used / max, MB | 1315.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.86% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 5.93% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 3 | 4.76% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 4 | 2.33% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.06% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 10904` |
| 6 | 1.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.86% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 8 | 1.29% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 10 | 1.18% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.14% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10175` |
| 12 | 1.05% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.02% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10811` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 15 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

