# Baseline run: `mobs-25k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25068 -> 25068) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 277.722 / 542.690 / 709.441 / 761.132 |
| MSPT mean, ms | 328.706 |
| TPS estimate (1000/p50) | 3.6 |
| heap used / max, MB | 1402.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.62% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.09% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 1.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8633` |
| 5 | 1.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 7 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 11 | 1.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.29% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 13 | 1.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 14 | 1.16% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

