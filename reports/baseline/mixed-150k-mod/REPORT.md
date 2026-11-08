# Baseline run: `mixed-150k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113167 -> 109515) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 829.999 / 952.640 / 1200.859 / 1313.042 |
| MSPT mean, ms | 854.213 |
| TPS estimate (1000/p50) | 1.2 |
| heap used / max, MB | 2290.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2731` |
| 2 | 3.29% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 3 | 3.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |
| 4 | 3.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10970` |
| 5 | 2.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.85% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.77% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 8 | 1.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.44% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.38% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.14% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

