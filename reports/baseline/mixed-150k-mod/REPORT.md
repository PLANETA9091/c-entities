# Baseline run: `mixed-150k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100578 -> 100578) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 270.848 / 326.285 / 376.111 / 450.458 |
| MSPT mean, ms | 275.962 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 1536.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 4 | 1.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.77% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6923` |
| 6 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.51% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.43% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6759` |
| 9 | 1.42% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 10 | 1.31% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6870` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1431` |
| 12 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.14% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 14 | 1.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.05% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

