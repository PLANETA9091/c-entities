# Baseline run: `spread-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97370 -> 97370) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 298.529 / 461.658 / 657.666 / 766.513 |
| MSPT mean, ms | 327.055 |
| TPS estimate (1000/p50) | 3.3 |
| heap used / max, MB | 1988.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.39% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.42% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8572` |
| 3 | 3.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8431` |
| 4 | 3.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5467` |
| 7 | 2.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8378` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4676` |
| 11 | 1.26% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5322` |
| 13 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.02% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.00% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

