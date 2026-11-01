# Baseline run: `mobs-25k-mod`

Server: `26.3-236-f119026 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25056 -> 25056) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 196.010 / 291.712 / 415.231 / 516.364 |
| MSPT mean, ms | 217.511 |
| TPS estimate (1000/p50) | 5.1 |
| heap used / max, MB | 1649.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 2 | 3.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 3 | 3.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.37% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 2.18% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 7 | 2.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 9 | 1.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8920` |
| 11 | 1.64% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9061` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4146` |
| 14 | 1.26% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8867` |
| 15 | 1.20% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |

