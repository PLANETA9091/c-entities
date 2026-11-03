# Baseline run: `mixed-150k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109735 -> 109735) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 312.158 / 403.991 / 529.817 / 573.868 |
| MSPT mean, ms | 331.865 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 1847.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2515` |
| 2 | 6.38% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 5.34% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2246` |
| 4 | 4.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6583` |
| 6 | 2.36% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10490` |
| 7 | 2.20% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9888` |
| 12 | 1.38% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 13 | 1.33% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2255` |
| 14 | 1.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.19% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

