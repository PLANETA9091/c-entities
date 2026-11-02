# Baseline run: `spread-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97436 -> 97436) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 257.024 / 363.781 / 503.807 / 659.466 |
| MSPT mean, ms | 270.545 |
| TPS estimate (1000/p50) | 3.9 |
| heap used / max, MB | 2285.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.82% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9898` |
| 2 | 3.72% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 2.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9395` |
| 5 | 2.88% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 2.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6241` |
| 7 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 2.15% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.66% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.22% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

