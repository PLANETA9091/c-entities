# Baseline run: `mobs-25k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25053 -> 25053) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 350.691 / 590.639 / 721.115 / 772.867 |
| MSPT mean, ms | 401.426 |
| TPS estimate (1000/p50) | 2.9 |
| heap used / max, MB | 1823.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.98% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2515` |
| 2 | 3.68% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.33% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2246` |
| 4 | 1.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.63% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10490` |
| 6 | 1.42% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 7 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.17% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2244` |
| 11 | 1.05% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9888` |
| 12 | 1.01% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 0.96% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1280` |
| 14 | 0.93% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 15 | 0.91% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

