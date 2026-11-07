# Baseline run: `mixed-150k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 112927 -> 109615) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 569.554 / 692.253 / 885.788 / 1004.379 |
| MSPT mean, ms | 598.862 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 2874.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 3 | 3.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 4 | 2.57% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 5 | 2.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 6 | 2.26% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.57% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.55% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 10 | 1.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 12 | 1.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.31% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 14 | 1.25% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 15 | 1.25% | `java.util.ImmutableCollections$ListItr.next() line: 397` |

