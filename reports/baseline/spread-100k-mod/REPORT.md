# Baseline run: `spread-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97330 -> 97330) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 151.880 / 255.522 / 360.769 / 462.290 |
| MSPT mean, ms | 174.678 |
| TPS estimate (1000/p50) | 6.6 |
| heap used / max, MB | 1850.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.95% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 3 | 3.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 3.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 7 | 2.05% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 1.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.74% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.98% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |

