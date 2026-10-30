# Baseline run: `mobs-25k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25054 -> 25054) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 315.199 / 522.422 / 643.634 / 859.731 |
| MSPT mean, ms | 353.086 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 1604.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.79% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 1.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 1.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8588` |
| 6 | 1.78% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 10 | 1.40% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8729` |
| 11 | 1.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 12 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.10% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 14 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5624` |

