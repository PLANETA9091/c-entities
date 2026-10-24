# Baseline run: `items-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54745 -> 54745) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 55.315 / 71.431 / 186.205 / 226.625 |
| MSPT mean, ms | 62.454 |
| TPS estimate (1000/p50) | 18.1 |
| heap used / max, MB | 1543.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.86% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.get(VarHandle, Object) line: 92` |
| 2 | 10.38% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2972` |
| 3 | 8.86% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 4 | 8.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3099` |
| 5 | 5.83% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 3633` |
| 6 | 4.93% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2962` |
| 7 | 4.43% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 8 | 4.01% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 9 | 3.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 10 | 2.18% | `jdk.internal.reflect.MethodHandleObjectFieldAccessorImpl.get(Object) line: 57` |
| 11 | 1.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1965` |
| 12 | 1.93% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2062` |
| 14 | 1.02% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2995` |
| 15 | 0.98% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |

