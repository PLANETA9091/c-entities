# Baseline run: `mixed-150k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100331 -> 100331) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 355.365 / 429.804 / 469.749 / 540.022 |
| MSPT mean, ms | 359.195 |
| TPS estimate (1000/p50) | 2.8 |
| heap used / max, MB | 1867.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 2 | 3.98% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 3 | 3.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3025` |
| 5 | 2.60% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2898` |
| 6 | 2.32% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 2.07% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 1.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.64% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2927` |
| 10 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.21% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 817` |
| 15 | 1.11% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |

