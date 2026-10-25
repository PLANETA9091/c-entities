# Baseline run: `mixed-150k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100565 -> 100565) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 239.144 / 276.894 / 327.359 / 383.245 |
| MSPT mean, ms | 242.228 |
| TPS estimate (1000/p50) | 4.2 |
| heap used / max, MB | 2094.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 2.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3314` |
| 5 | 2.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 6 | 2.22% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.00% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 8 | 1.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3460` |
| 9 | 1.87% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4210` |
| 10 | 1.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.68% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1494` |
| 12 | 1.66% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3578` |
| 13 | 1.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 15 | 1.36% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 4124` |

