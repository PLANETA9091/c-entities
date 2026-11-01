# Baseline run: `spread-100k-mod`

Server: `26.3-235-bb10cf0 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97439 -> 97439) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 289.246 / 444.423 / 562.475 / 709.852 |
| MSPT mean, ms | 317.508 |
| TPS estimate (1000/p50) | 3.5 |
| heap used / max, MB | 1952.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.36% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.47% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8977` |
| 3 | 3.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 4 | 2.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.60% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 9 | 1.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5145` |
| 10 | 1.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5727` |
| 13 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 0.99% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

