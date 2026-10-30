# Baseline run: `mobs-25k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25050 -> 25050) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 309.416 / 591.144 / 779.471 / 874.575 |
| MSPT mean, ms | 359.988 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 1646.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.48% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.39% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 1.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8593` |
| 6 | 1.52% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.45% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8734` |
| 9 | 1.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5629` |
| 10 | 1.17% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 1.11% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.05% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.94% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 15 | 0.89% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

