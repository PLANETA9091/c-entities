# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25054 -> 25054) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 282.038 / 476.421 / 583.503 / 689.249 |
| MSPT mean, ms | 322.133 |
| TPS estimate (1000/p50) | 3.5 |
| heap used / max, MB | 1292.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.45% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 2.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 2.00% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9149` |
| 6 | 1.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9008` |
| 7 | 1.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 13 | 1.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 1.00% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |

