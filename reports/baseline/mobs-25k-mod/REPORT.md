# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25071 -> 25071) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 245.520 / 380.080 / 511.813 / 663.444 |
| MSPT mean, ms | 276.807 |
| TPS estimate (1000/p50) | 4.1 |
| heap used / max, MB | 1174.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 3 | 2.12% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 4 | 2.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9116` |
| 5 | 2.05% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.01% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.79% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9257` |
| 9 | 1.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9063` |
| 12 | 1.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.37% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 1.28% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 15 | 1.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

