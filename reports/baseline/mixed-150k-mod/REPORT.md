# Baseline run: `mixed-150k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100613 -> 100613) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 274.307 / 332.369 / 396.353 / 437.976 |
| MSPT mean, ms | 280.108 |
| TPS estimate (1000/p50) | 3.6 |
| heap used / max, MB | 2958.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.97% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 4510` |
| 4 | 2.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3488` |
| 7 | 2.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 2.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3670` |
| 9 | 1.87% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 12 | 1.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3489` |
| 13 | 1.38% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.36% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 15 | 1.32% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 4373` |

