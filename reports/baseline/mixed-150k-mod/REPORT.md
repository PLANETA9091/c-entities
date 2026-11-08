# Baseline run: `mixed-150k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113352 -> 109569) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 858.572 / 1002.456 / 1247.397 / 1435.631 |
| MSPT mean, ms | 886.127 |
| TPS estimate (1000/p50) | 1.2 |
| heap used / max, MB | 1869.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.07% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 2 | 3.37% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 3 | 3.16% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 4 | 3.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10965` |
| 5 | 3.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7102` |
| 6 | 2.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.09% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2726` |
| 8 | 1.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.53% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.24% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.14% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.11% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |

