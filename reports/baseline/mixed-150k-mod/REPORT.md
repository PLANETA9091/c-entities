# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109795 -> 109795) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 229.806 / 325.945 / 412.592 / 472.048 |
| MSPT mean, ms | 252.061 |
| TPS estimate (1000/p50) | 4.4 |
| heap used / max, MB | 1696.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.65% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8935` |
| 2 | 4.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.37% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9129` |
| 4 | 2.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.57% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.47% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5857` |
| 7 | 2.38% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8988` |
| 9 | 1.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6011` |
| 10 | 1.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.64% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 15 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

