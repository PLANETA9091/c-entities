# Baseline run: `mixed-150k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109730 -> 109730) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 246.647 / 338.537 / 447.273 / 535.165 |
| MSPT mean, ms | 267.053 |
| TPS estimate (1000/p50) | 4.1 |
| heap used / max, MB | 1760.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8242` |
| 3 | 2.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.96% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5278` |
| 5 | 2.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.20% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8189` |
| 9 | 2.09% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 2.09% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5133` |
| 11 | 2.08% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.84% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3579` |
| 13 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 1.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

