# Baseline run: `mobs-25k-mod`

Server: `26.3-235-bb10cf0 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25060 -> 25060) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 216.807 / 317.503 / 477.398 / 532.630 |
| MSPT mean, ms | 240.074 |
| TPS estimate (1000/p50) | 4.6 |
| heap used / max, MB | 1213.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 2 | 3.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.04% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.97% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 2.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 2.06% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8977` |
| 7 | 1.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 9 | 1.69% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 11 | 1.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 12 | 1.50% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 13 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.24% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4116` |

