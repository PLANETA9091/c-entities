# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109813 -> 109813) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 269.588 / 378.518 / 554.702 / 643.671 |
| MSPT mean, ms | 295.803 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 1948.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6262` |
| 2 | 3.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.91% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 2.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5494` |
| 5 | 2.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9532` |
| 6 | 2.13% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10134` |
| 8 | 1.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.51% | `java.lang.ref.Reference.get() line: 361` |
| 13 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |

