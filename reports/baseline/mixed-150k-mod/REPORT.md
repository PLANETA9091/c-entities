# Baseline run: `mixed-150k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109747 -> 109747) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 241.467 / 333.698 / 439.856 / 482.046 |
| MSPT mean, ms | 260.955 |
| TPS estimate (1000/p50) | 4.1 |
| heap used / max, MB | 2097.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.18% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8699` |
| 3 | 3.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8558` |
| 4 | 2.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5594` |
| 5 | 2.97% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.75% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 2.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 2.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 2.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8505` |
| 12 | 1.95% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5449` |
| 13 | 1.54% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 1.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

