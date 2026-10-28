# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109678 -> 109678) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 281.879 / 382.902 / 487.285 / 702.062 |
| MSPT mean, ms | 301.060 |
| TPS estimate (1000/p50) | 3.5 |
| heap used / max, MB | 2011.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7778` |
| 3 | 2.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4838` |
| 4 | 2.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 6 | 2.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.99% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.97% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4693` |
| 10 | 1.84% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7725` |
| 11 | 1.77% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 12 | 1.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

