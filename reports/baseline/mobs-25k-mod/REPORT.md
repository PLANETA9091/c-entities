# Baseline run: `mobs-25k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25058 -> 25058) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 273.434 / 514.263 / 688.043 / 782.592 |
| MSPT mean, ms | 321.705 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 1574.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.49% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 1.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 1.70% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 5 | 1.55% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 6 | 1.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 7 | 1.46% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.29% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 10 | 1.14% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 11 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.99% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 14 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.85% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

