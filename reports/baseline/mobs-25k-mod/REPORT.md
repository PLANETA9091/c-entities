# Baseline run: `mobs-25k-mod`

Server: `26.3-235-bb10cf0 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25063 -> 25063) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 232.806 / 355.097 / 463.083 / 537.543 |
| MSPT mean, ms | 256.570 |
| TPS estimate (1000/p50) | 4.3 |
| heap used / max, MB | 1332.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.45% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.41% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.50% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 9 | 1.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 9044` |
| 11 | 1.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 12 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 1.25% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 14 | 1.21% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 15 | 1.18% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |

