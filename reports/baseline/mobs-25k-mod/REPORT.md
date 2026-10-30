# Baseline run: `mobs-25k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25073 -> 25073) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 269.546 / 556.349 / 673.904 / 739.413 |
| MSPT mean, ms | 317.070 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 1666.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.30% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.00% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 7 | 1.73% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8516` |
| 9 | 1.52% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8657` |
| 10 | 1.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 11 | 1.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5552` |
| 12 | 1.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.07% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

