# Baseline run: `spread-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 95950 -> 95591) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 367.770 / 474.472 / 535.700 / 651.768 |
| MSPT mean, ms | 378.468 |
| TPS estimate (1000/p50) | 2.7 |
| heap used / max, MB | 2255.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.00% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 1.91% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.90% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 8 | 1.55% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 9 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 10 | 1.51% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 11 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 13 | 1.16% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 14 | 1.06% | `java.util.HashMap.getNode(Object) line: 585` |
| 15 | 0.94% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |

