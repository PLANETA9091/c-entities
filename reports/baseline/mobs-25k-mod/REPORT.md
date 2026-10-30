# Baseline run: `mobs-25k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25079 -> 25079) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 223.825 / 347.659 / 471.890 / 509.276 |
| MSPT mean, ms | 253.560 |
| TPS estimate (1000/p50) | 4.5 |
| heap used / max, MB | 1194.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.76% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 2.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 7 | 1.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 10 | 1.40% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 12 | 1.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 13 | 1.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 14 | 1.17% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 15 | 1.12% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |

