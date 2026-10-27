# Baseline run: `spread-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97377 -> 97377) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 148.920 / 258.604 / 405.113 / 528.276 |
| MSPT mean, ms | 171.007 |
| TPS estimate (1000/p50) | 6.7 |
| heap used / max, MB | 2324.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 3 | 2.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 2.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 6 | 2.03% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.69% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 9 | 1.55% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 10 | 1.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.36% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.07% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.99% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 15 | 0.94% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

