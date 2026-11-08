# Baseline run: `mixed-150k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113333 -> 109777) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 1374.200 / 1548.426 / 1716.062 / 1776.777 |
| MSPT mean, ms | 1399.817 |
| TPS estimate (1000/p50) | 0.7 |
| heap used / max, MB | 1820.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7120` |
| 2 | 3.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10983` |
| 3 | 2.82% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 4 | 2.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2744` |
| 6 | 1.81% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.64% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 8 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.55% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.46% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.11% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |

