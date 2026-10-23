# Baseline run: `spread-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 96940 -> 96026) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 307.950 / 380.476 / 451.232 / 534.740 |
| MSPT mean, ms | 320.240 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 2168.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 5 | 2.13% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 2.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 7 | 1.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 8 | 1.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 11 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 13 | 1.25% | `java.util.HashMap.getNode(Object) line: 585` |
| 14 | 1.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.11% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |

