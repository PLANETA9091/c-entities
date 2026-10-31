# Baseline run: `mobs-25k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25058 -> 25058) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 223.102 / 326.258 / 467.804 / 560.436 |
| MSPT mean, ms | 248.118 |
| TPS estimate (1000/p50) | 4.5 |
| heap used / max, MB | 1543.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 2.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.31% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 2.10% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 2.07% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 8 | 1.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 9 | 1.72% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 10 | 1.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.54% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.41% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 15 | 1.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |

