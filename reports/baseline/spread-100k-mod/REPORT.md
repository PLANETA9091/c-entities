# Baseline run: `spread-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97422 -> 97422) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 260.499 / 420.130 / 530.829 / 699.754 |
| MSPT mean, ms | 290.035 |
| TPS estimate (1000/p50) | 3.8 |
| heap used / max, MB | 2125.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.87% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10199` |
| 2 | 3.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6311` |
| 5 | 2.11% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 1.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5543` |
| 8 | 1.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9597` |
| 9 | 1.25% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 10 | 1.22% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 11 | 1.17% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 12 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.97% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 0.95% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |

