# Baseline run: `spread-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97403 -> 97403) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 149.675 / 235.954 / 364.278 / 548.381 |
| MSPT mean, ms | 169.975 |
| TPS estimate (1000/p50) | 6.7 |
| heap used / max, MB | 2458.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.74% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7695` |
| 3 | 3.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7554` |
| 5 | 2.70% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4614` |
| 7 | 2.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7501` |
| 8 | 2.24% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4469` |
| 9 | 2.09% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.87% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.23% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

