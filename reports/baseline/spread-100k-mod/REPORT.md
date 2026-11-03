# Baseline run: `spread-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97448 -> 97448) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 287.309 / 465.194 / 572.764 / 822.899 |
| MSPT mean, ms | 317.080 |
| TPS estimate (1000/p50) | 3.5 |
| heap used / max, MB | 2886.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.41% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10235` |
| 2 | 3.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6347` |
| 5 | 2.15% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 1.73% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9633` |
| 7 | 1.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.20% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 10 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.18% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.16% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 14 | 1.04% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 15 | 1.04% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |

