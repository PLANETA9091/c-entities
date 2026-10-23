# Baseline run: `spread-100k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 95568 -> 95538) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 272.140 / 330.648 / 463.039 / 523.585 |
| MSPT mean, ms | 285.793 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 1893.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 4 | 3.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 2.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.48% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.07% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 13 | 1.02% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 14 | 0.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 15 | 0.89% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |

