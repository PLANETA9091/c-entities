# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109704 -> 109704) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 343.239 / 481.721 / 627.732 / 721.716 |
| MSPT mean, ms | 367.674 |
| TPS estimate (1000/p50) | 2.9 |
| heap used / max, MB | 2340.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 17.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.70% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7863` |
| 5 | 2.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 6 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 8 | 1.86% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.65% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.55% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4637` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 12 | 1.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.05% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.02% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

