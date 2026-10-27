# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25065 -> 25065) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 165.964 / 279.896 / 430.439 / 503.012 |
| MSPT mean, ms | 185.040 |
| TPS estimate (1000/p50) | 6.0 |
| heap used / max, MB | 1681.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 1.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 3 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 1.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 7 | 1.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.31% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.01% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 12 | 0.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 13 | 0.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 14 | 0.82% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 15 | 0.79% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |

