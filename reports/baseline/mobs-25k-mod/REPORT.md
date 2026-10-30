# Baseline run: `mobs-25k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25070 -> 25070) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 267.616 / 437.723 / 543.416 / 649.569 |
| MSPT mean, ms | 299.152 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 1333.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.88% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.73% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8572` |
| 7 | 1.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8431` |
| 8 | 1.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 10 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.22% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.02% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 0.98% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 14 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.97% | `net.minecraft.world.entity.Entity.getId() line: 651` |

