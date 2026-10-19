# Baseline run: `spread-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97515 -> 97515) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 415.174 / 479.902 / 569.700 / 590.037 |
| MSPT mean, ms | 419.883 |
| TPS estimate (1000/p50) | 2.4 |
| heap used / max, MB | 2318.1 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 20.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 4 | 2.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 1.68% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 7 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.30% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 9 | 1.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.18% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 11 | 1.10% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 12 | 1.08% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 13 | 1.04% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 14 | 1.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |

