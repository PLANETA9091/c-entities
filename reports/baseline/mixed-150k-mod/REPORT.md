# Baseline run: `mixed-150k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100358 -> 100358) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 300.484 / 357.751 / 412.446 / 442.374 |
| MSPT mean, ms | 302.573 |
| TPS estimate (1000/p50) | 3.3 |
| heap used / max, MB | 1905.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.15% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.get(VarHandle, Object) line: 92` |
| 2 | 3.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3099` |
| 4 | 2.74% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1467` |
| 5 | 2.73% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 6 | 2.65% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2962` |
| 7 | 2.27% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2972` |
| 8 | 2.09% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 11 | 1.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.26% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 14 | 1.26% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 3633` |
| 15 | 1.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

