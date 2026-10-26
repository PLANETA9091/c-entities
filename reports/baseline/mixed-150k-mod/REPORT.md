# Baseline run: `mixed-150k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100580 -> 100580) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 265.826 / 330.720 / 383.523 / 461.028 |
| MSPT mean, ms | 274.099 |
| TPS estimate (1000/p50) | 3.8 |
| heap used / max, MB | 2705.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.13% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6921` |
| 4 | 1.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 5 | 1.70% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1431` |
| 7 | 1.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.46% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6757` |
| 11 | 1.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.29% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6868` |
| 13 | 1.08% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.08% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 15 | 1.05% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |

