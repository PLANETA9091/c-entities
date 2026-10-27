# Baseline run: `spread-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97359 -> 97359) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 152.369 / 240.666 / 376.433 / 463.728 |
| MSPT mean, ms | 171.203 |
| TPS estimate (1000/p50) | 6.6 |
| heap used / max, MB | 3037.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.58% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7357` |
| 3 | 3.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 4 | 2.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 5 | 2.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.42% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 8 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 2.09% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4131` |
| 11 | 2.03% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.73% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.22% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

