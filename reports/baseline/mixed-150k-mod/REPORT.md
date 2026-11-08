# Baseline run: `mixed-150k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 112929 -> 109615) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 1639.690 / 1923.249 / 2119.448 / 2270.754 |
| MSPT mean, ms | 1693.424 |
| TPS estimate (1000/p50) | 0.6 |
| heap used / max, MB | 1886.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7120` |
| 2 | 3.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10983` |
| 3 | 3.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.61% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 5 | 2.31% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 6 | 2.31% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 11531` |
| 7 | 1.82% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.72% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.58% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.38% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.24% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 1.17% | `java.util.ImmutableCollections$ListItr.next() line: 397` |

