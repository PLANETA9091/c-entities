# Baseline run: `spread-100k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 95663 -> 95632) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 257.439 / 310.294 / 433.689 / 464.401 |
| MSPT mean, ms | 269.545 |
| TPS estimate (1000/p50) | 3.9 |
| heap used / max, MB | 1947.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.49% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 3 | 3.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 3.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 10 | 1.09% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.08% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.05% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 13 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 14 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 15 | 0.87% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |

