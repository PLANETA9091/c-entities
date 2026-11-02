# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25073 -> 25073) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 309.733 / 561.641 / 690.990 / 807.728 |
| MSPT mean, ms | 362.589 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 1484.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.39% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 1.97% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9334` |
| 5 | 1.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 6 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9193` |
| 8 | 1.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 10 | 1.12% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 11 | 1.05% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.03% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.91% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 15 | 0.90% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |

