# Baseline run: `mobs-25k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25060 -> 25060) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 272.964 / 510.281 / 653.797 / 795.467 |
| MSPT mean, ms | 320.219 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 1354.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.09% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 2.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.96% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8696` |
| 8 | 1.46% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8837` |
| 9 | 1.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5732` |
| 10 | 1.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 11 | 1.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.25% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 13 | 1.15% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.14% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |

