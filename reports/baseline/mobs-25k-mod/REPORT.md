# Baseline run: `mobs-25k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25061 -> 25061) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 226.527 / 333.178 / 481.974 / 578.492 |
| MSPT mean, ms | 250.124 |
| TPS estimate (1000/p50) | 4.4 |
| heap used / max, MB | 1574.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.86% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.40% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 2.00% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8474` |
| 8 | 1.67% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 9 | 1.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 10 | 1.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5510` |
| 12 | 1.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 1.47% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8615` |
| 14 | 1.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.29% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |

