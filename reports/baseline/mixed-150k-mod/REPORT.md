# Baseline run: `mixed-150k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100591 -> 100591) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 227.173 / 263.545 / 313.443 / 364.110 |
| MSPT mean, ms | 233.625 |
| TPS estimate (1000/p50) | 4.4 |
| heap used / max, MB | 2026.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.82% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7336` |
| 3 | 3.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 4 | 2.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7195` |
| 6 | 2.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4255` |
| 8 | 2.05% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4110` |
| 10 | 1.70% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7142` |
| 11 | 1.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 13 | 1.21% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 14 | 1.20% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 1.12% | `java.util.EnumMap.get(Object) line: 248` |

