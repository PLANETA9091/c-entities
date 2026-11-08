# Baseline run: `mixed-150k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113031 -> 109667) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 743.533 / 883.812 / 1114.874 / 1266.474 |
| MSPT mean, ms | 775.220 |
| TPS estimate (1000/p50) | 1.3 |
| heap used / max, MB | 1949.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 2 | 3.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 3 | 3.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2712` |
| 5 | 1.92% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 6 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.60% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 8 | 1.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 10 | 1.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.29% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.09% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

