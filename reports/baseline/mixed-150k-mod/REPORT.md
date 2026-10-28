# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109708 -> 109708) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 433.925 / 602.489 / 816.390 / 899.474 |
| MSPT mean, ms | 453.783 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 1767.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 16.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.64% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7863` |
| 4 | 2.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 5 | 2.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 2.12% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.97% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 9 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.10% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4637` |
| 12 | 1.07% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 13 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 15 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

