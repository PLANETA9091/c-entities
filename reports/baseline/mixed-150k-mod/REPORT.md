# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109777 -> 109777) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 263.232 / 381.175 / 536.937 / 651.528 |
| MSPT mean, ms | 291.582 |
| TPS estimate (1000/p50) | 3.8 |
| heap used / max, MB | 1905.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6311` |
| 2 | 3.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.32% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.24% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10199` |
| 5 | 2.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5543` |
| 6 | 2.06% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.98% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9597` |
| 8 | 1.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |

