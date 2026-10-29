# Baseline run: `spread-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97446 -> 97446) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 182.633 / 301.215 / 450.790 / 681.555 |
| MSPT mean, ms | 207.822 |
| TPS estimate (1000/p50) | 5.5 |
| heap used / max, MB | 2767.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4886` |
| 2 | 3.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7826` |
| 4 | 2.82% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.44% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7967` |
| 7 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.11% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7773` |
| 9 | 1.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.56% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4741` |
| 11 | 1.53% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4168` |
| 14 | 1.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.14% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

