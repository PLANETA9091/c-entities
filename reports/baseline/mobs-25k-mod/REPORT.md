# Baseline run: `mobs-25k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25056 -> 25056) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 280.513 / 462.095 / 606.934 / 679.517 |
| MSPT mean, ms | 316.874 |
| TPS estimate (1000/p50) | 3.6 |
| heap used / max, MB | 1456.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.00% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8383` |
| 7 | 1.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8242` |
| 9 | 1.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 10 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 14 | 1.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.00% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |

