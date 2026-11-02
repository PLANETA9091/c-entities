# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25071 -> 25071) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 239.505 / 375.140 / 481.561 / 570.480 |
| MSPT mean, ms | 268.528 |
| TPS estimate (1000/p50) | 4.2 |
| heap used / max, MB | 1724.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.32% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 3 | 3.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.26% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 2.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 6 | 2.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.11% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 9 | 2.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9126` |
| 11 | 1.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4352` |
| 13 | 1.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 15 | 1.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |

