# Baseline run: `spread-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97352 -> 97352) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 166.959 / 267.973 / 363.725 / 475.969 |
| MSPT mean, ms | 190.359 |
| TPS estimate (1000/p50) | 6.0 |
| heap used / max, MB | 1817.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 2 | 3.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3065` |
| 3 | 3.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.92% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2938` |
| 5 | 2.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 6 | 2.56% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1462` |
| 7 | 2.29% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2967` |
| 8 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.71% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3067` |
| 14 | 1.19% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

