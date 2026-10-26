# Baseline run: `mixed-150k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100595 -> 100595) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 286.749 / 349.419 / 406.139 / 482.451 |
| MSPT mean, ms | 290.879 |
| TPS estimate (1000/p50) | 3.5 |
| heap used / max, MB | 1785.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.09% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.16% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5454` |
| 3 | 2.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.17% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3627` |
| 6 | 1.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3791` |
| 8 | 1.69% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.64% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5413` |
| 10 | 1.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5317` |
| 13 | 1.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.23% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |

