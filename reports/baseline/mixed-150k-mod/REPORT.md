# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109770 -> 109770) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 317.432 / 413.025 / 530.595 / 550.423 |
| MSPT mean, ms | 337.775 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 1713.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.29% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7826` |
| 4 | 3.15% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7967` |
| 5 | 3.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4886` |
| 6 | 2.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.02% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.89% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4741` |
| 10 | 1.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7773` |
| 11 | 1.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4168` |
| 13 | 1.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

