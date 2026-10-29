# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109697 -> 109697) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 326.102 / 416.430 / 546.909 / 610.549 |
| MSPT mean, ms | 341.071 |
| TPS estimate (1000/p50) | 3.1 |
| heap used / max, MB | 1713.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7783` |
| 3 | 2.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.15% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.82% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4751` |
| 6 | 1.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7836` |
| 7 | 1.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4896` |
| 8 | 1.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.37% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 13 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.15% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 1.03% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |

