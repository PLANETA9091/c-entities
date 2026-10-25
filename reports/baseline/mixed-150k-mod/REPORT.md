# Baseline run: `mixed-150k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100327 -> 100327) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 321.187 / 399.958 / 441.505 / 519.888 |
| MSPT mean, ms | 325.811 |
| TPS estimate (1000/p50) | 3.1 |
| heap used / max, MB | 2093.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 3844` |
| 2 | 3.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.11% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1484` |
| 4 | 2.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3190` |
| 5 | 2.42% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 2.36% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.78% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.43% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3053` |
| 9 | 1.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3063` |
| 10 | 1.38% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 3818` |
| 11 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1413` |
| 13 | 1.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 15 | 1.09% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |

