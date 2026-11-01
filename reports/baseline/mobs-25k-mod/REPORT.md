# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25051 -> 25051) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 308.893 / 577.272 / 734.276 / 845.389 |
| MSPT mean, ms | 363.415 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 1878.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.70% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.47% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.93% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.83% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9129` |
| 7 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8988` |
| 9 | 1.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 1.13% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.10% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 1.07% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 14 | 0.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |

