# Baseline run: `spread-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 91706 -> 91281) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 444.854 / 629.215 / 699.379 / 783.371 |
| MSPT mean, ms | 474.000 |
| TPS estimate (1000/p50) | 2.2 |
| heap used / max, MB | 2390.3 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.21% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.98% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 5 | 1.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.41% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 9 | 1.37% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 10 | 1.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 12 | 1.19% | `java.util.HashMap.getNode(Object) line: 585` |
| 13 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 14 | 1.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 730` |

