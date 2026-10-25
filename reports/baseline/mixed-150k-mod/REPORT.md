# Baseline run: `mixed-150k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100537 -> 100537) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 233.971 / 286.404 / 334.588 / 376.850 |
| MSPT mean, ms | 240.758 |
| TPS estimate (1000/p50) | 4.3 |
| heap used / max, MB | 2209.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.38% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.75% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3624` |
| 5 | 2.62% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5463` |
| 6 | 2.60% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 7 | 2.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 2.04% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 1.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3806` |
| 10 | 1.82% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 11 | 1.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.78% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5422` |
| 13 | 1.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.46% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5326` |
| 15 | 1.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |

