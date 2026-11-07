# Baseline run: `mixed-150k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113289 -> 109713) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 821.613 / 953.444 / 1191.463 / 1289.269 |
| MSPT mean, ms | 848.936 |
| TPS estimate (1000/p50) | 1.2 |
| heap used / max, MB | 2037.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2712` |
| 2 | 3.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 3 | 3.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 4 | 2.94% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 5 | 2.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.20% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 7 | 1.63% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 8 | 1.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.30% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.15% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 1.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

