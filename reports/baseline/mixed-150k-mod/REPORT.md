# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109686 -> 109686) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 488.916 / 648.213 / 851.938 / 994.292 |
| MSPT mean, ms | 506.819 |
| TPS estimate (1000/p50) | 2.0 |
| heap used / max, MB | 1736.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.75% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7776` |
| 3 | 2.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7635` |
| 4 | 2.44% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4695` |
| 6 | 2.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.94% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.63% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3986` |
| 11 | 1.14% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7582` |
| 12 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.01% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4550` |

