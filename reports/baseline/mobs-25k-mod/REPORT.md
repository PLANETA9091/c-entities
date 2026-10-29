# Baseline run: `mobs-25k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25066 -> 25066) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 176.383 / 320.755 / 530.198 / 574.456 |
| MSPT mean, ms | 200.020 |
| TPS estimate (1000/p50) | 5.7 |
| heap used / max, MB | 1488.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 2.17% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 2.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4932` |
| 6 | 2.02% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.94% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.66% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 9 | 1.53% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 1.51% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.44% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 12 | 1.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7872` |
| 13 | 1.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2054` |
| 15 | 1.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |

