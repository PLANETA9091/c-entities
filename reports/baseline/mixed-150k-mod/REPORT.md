# Baseline run: `mixed-150k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100556 -> 100556) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 294.249 / 349.120 / 406.033 / 462.597 |
| MSPT mean, ms | 295.372 |
| TPS estimate (1000/p50) | 3.4 |
| heap used / max, MB | 1541.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.14% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5480` |
| 4 | 2.59% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.55% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3671` |
| 6 | 2.13% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3817` |
| 8 | 1.79% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 11 | 1.36% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5343` |
| 12 | 1.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5439` |
| 14 | 1.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.21% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |

