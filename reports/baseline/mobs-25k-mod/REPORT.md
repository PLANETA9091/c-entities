# Baseline run: `mobs-25k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25067 -> 25067) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 264.276 / 439.787 / 560.752 / 629.711 |
| MSPT mean, ms | 297.761 |
| TPS estimate (1000/p50) | 3.8 |
| heap used / max, MB | 1502.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.59% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.26% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8840` |
| 4 | 2.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.01% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 6 | 1.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8699` |
| 10 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 12 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.07% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 14 | 1.03% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 15 | 0.96% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

