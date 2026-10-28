# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109647 -> 109647) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 455.768 / 591.594 / 798.572 / 922.441 |
| MSPT mean, ms | 478.513 |
| TPS estimate (1000/p50) | 2.2 |
| heap used / max, MB | 1830.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 17.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.56% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7721` |
| 4 | 2.52% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7862` |
| 5 | 2.12% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4781` |
| 7 | 1.98% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.71% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4072` |
| 12 | 1.06% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7668` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4636` |
| 14 | 0.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

