# Baseline run: `mixed-150k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100575 -> 100575) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 255.691 / 310.571 / 361.033 / 389.276 |
| MSPT mean, ms | 262.476 |
| TPS estimate (1000/p50) | 3.9 |
| heap used / max, MB | 1934.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.08% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6703` |
| 4 | 2.94% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6650` |
| 5 | 2.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 8 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.51% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4058` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 12 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6539` |
| 14 | 1.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.03% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

