# Baseline run: `mixed-150k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100565 -> 100565) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 211.521 / 262.513 / 313.308 / 509.393 |
| MSPT mean, ms | 219.645 |
| TPS estimate (1000/p50) | 4.7 |
| heap used / max, MB | 2586.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.19% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.22% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 3.20% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6473` |
| 5 | 2.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 6 | 2.05% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 7 | 1.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 8 | 1.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 9 | 1.61% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 10 | 1.55% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 11 | 1.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 14 | 1.40% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6309` |
| 15 | 1.15% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4052` |

