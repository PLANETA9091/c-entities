# Baseline run: `spread-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97418 -> 97418) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 159.615 / 264.883 / 389.513 / 532.362 |
| MSPT mean, ms | 182.418 |
| TPS estimate (1000/p50) | 6.3 |
| heap used / max, MB | 2187.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.38% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.58% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7776` |
| 3 | 3.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7635` |
| 4 | 2.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4695` |
| 5 | 2.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.17% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4550` |
| 9 | 2.11% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7582` |
| 10 | 1.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.47% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 14 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

