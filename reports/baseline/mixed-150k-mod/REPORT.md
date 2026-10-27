# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100603 -> 100603) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 253.336 / 334.402 / 438.403 / 500.428 |
| MSPT mean, ms | 265.847 |
| TPS estimate (1000/p50) | 3.9 |
| heap used / max, MB | 1518.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7652` |
| 3 | 2.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.73% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7511` |
| 5 | 2.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4571` |
| 6 | 2.49% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.03% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7458` |
| 9 | 1.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.69% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4426` |
| 11 | 1.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 12 | 1.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 13 | 1.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.39% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1764` |

