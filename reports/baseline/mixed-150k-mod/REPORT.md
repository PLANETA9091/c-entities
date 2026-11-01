# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109782 -> 109782) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 228.635 / 328.617 / 455.086 / 533.804 |
| MSPT mean, ms | 250.263 |
| TPS estimate (1000/p50) | 4.4 |
| heap used / max, MB | 1672.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.17% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9080` |
| 3 | 3.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5962` |
| 4 | 2.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8939` |
| 5 | 2.88% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8886` |
| 9 | 2.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 2.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 2.11% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5808` |
| 12 | 1.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |

