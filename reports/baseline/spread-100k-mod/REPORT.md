# Baseline run: `spread-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97449 -> 97449) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 311.157 / 512.162 / 668.542 / 885.825 |
| MSPT mean, ms | 345.307 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 2400.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.98% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10099` |
| 2 | 3.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6222` |
| 5 | 2.16% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.76% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9497` |
| 7 | 1.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5454` |
| 8 | 1.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.44% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.08% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.07% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.04% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 15 | 1.03% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

