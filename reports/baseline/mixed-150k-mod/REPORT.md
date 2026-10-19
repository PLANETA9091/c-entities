# Baseline run: `mixed-150k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110370 -> 110360) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 740.523 / 804.007 / 859.236 / 921.180 |
| MSPT mean, ms | 740.930 |
| TPS estimate (1000/p50) | 1.4 |
| heap used / max, MB | 1929.2 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 30.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 2.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.89% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 1.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 8 | 1.26% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 1.22% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 10 | 1.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 11 | 1.17% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 12 | 1.16% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 13 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.94% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 0.85% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |

