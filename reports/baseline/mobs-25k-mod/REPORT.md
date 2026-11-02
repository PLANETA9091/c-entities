# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25066 -> 25066) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 287.958 / 460.711 / 556.079 / 635.833 |
| MSPT mean, ms | 326.053 |
| TPS estimate (1000/p50) | 3.5 |
| heap used / max, MB | 1142.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.06% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9267` |
| 4 | 1.93% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.81% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9126` |
| 7 | 1.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 10 | 1.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 11 | 1.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |

