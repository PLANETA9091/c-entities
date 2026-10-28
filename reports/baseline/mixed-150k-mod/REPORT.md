# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109731 -> 109731) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 344.351 / 492.302 / 631.612 / 718.303 |
| MSPT mean, ms | 369.679 |
| TPS estimate (1000/p50) | 2.9 |
| heap used / max, MB | 1856.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 16.49% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.00% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7770` |
| 4 | 2.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7629` |
| 7 | 1.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4689` |
| 8 | 1.70% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.65% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4544` |
| 10 | 1.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7576` |
| 11 | 1.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.18% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.13% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.09% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

