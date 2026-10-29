# Baseline run: `mobs-25k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25061 -> 25061) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 220.782 / 336.330 / 462.413 / 514.311 |
| MSPT mean, ms | 245.106 |
| TPS estimate (1000/p50) | 4.5 |
| heap used / max, MB | 1607.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 2 | 3.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 3 | 3.11% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.52% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.92% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5278` |
| 9 | 1.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 10 | 1.52% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 11 | 1.52% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.47% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8383` |
| 13 | 1.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8242` |
| 14 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 1.21% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |

