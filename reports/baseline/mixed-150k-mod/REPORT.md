# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109715 -> 109715) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 424.467 / 568.181 / 769.259 / 888.874 |
| MSPT mean, ms | 445.467 |
| TPS estimate (1000/p50) | 2.4 |
| heap used / max, MB | 1824.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 20.22% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7656` |
| 4 | 2.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.68% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7797` |
| 8 | 1.58% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4716` |
| 10 | 1.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.02% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7603` |
| 13 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4571` |
| 15 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4007` |

