# Baseline run: `spread-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97386 -> 97386) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 165.587 / 259.567 / 347.889 / 471.118 |
| MSPT mean, ms | 184.313 |
| TPS estimate (1000/p50) | 6.0 |
| heap used / max, MB | 2041.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 2 | 4.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3025` |
| 3 | 3.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.50% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2898` |
| 5 | 2.22% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2927` |
| 6 | 2.18% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 7 | 2.09% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 2.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.65% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3027` |
| 13 | 1.25% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 14 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |

