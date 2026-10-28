# Baseline run: `spread-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97353 -> 97353) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 157.711 / 241.244 / 339.523 / 410.430 |
| MSPT mean, ms | 171.551 |
| TPS estimate (1000/p50) | 6.3 |
| heap used / max, MB | 2536.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7576` |
| 3 | 2.59% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7770` |
| 4 | 2.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7629` |
| 7 | 2.07% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.92% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.63% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4544` |
| 12 | 1.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 13 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.03% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 15 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

