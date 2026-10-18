# Baseline run: `mixed-150k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110343 -> 110343) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 676.202 / 720.070 / 767.849 / 796.980 |
| MSPT mean, ms | 670.253 |
| TPS estimate (1000/p50) | 1.5 |
| heap used / max, MB | 2815.0 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 8.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 6 | 1.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 8 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 0.81% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 11 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 12 | 0.77% | `java.util.HashMap.getNode(Object) line: 585` |
| 13 | 0.75% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 14 | 0.75% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 15 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |

