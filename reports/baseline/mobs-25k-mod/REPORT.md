# Baseline run: `mobs-25k-mod`

Server: `26.3-235-bb10cf0 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25053 -> 25053) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 269.550 / 528.273 / 658.494 / 719.135 |
| MSPT mean, ms | 317.938 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 1714.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.59% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8965` |
| 6 | 1.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 8 | 1.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.14% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 12 | 1.09% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 1.03% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 14 | 0.97% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 15 | 0.92% | `net.minecraft.world.entity.Entity.getId() line: 651` |

