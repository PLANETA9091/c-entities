# Baseline run: `mobs-25k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25057 -> 25057) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 214.275 / 358.018 / 557.620 / 606.704 |
| MSPT mean, ms | 250.028 |
| TPS estimate (1000/p50) | 4.7 |
| heap used / max, MB | 1619.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 2 | 2.94% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 2.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 2.12% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 7 | 2.09% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.91% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.56% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 10 | 1.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 12 | 1.26% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8691` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3983` |
| 15 | 1.20% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8638` |

