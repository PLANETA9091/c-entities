# Baseline run: `mixed-150k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100553 -> 100553) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 289.555 / 360.787 / 419.858 / 445.125 |
| MSPT mean, ms | 300.616 |
| TPS estimate (1000/p50) | 3.5 |
| heap used / max, MB | 2323.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.44% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7317` |
| 3 | 2.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7176` |
| 4 | 2.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.00% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4236` |
| 9 | 1.86% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.70% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7123` |
| 12 | 1.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4091` |
| 13 | 1.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1464` |
| 14 | 1.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

