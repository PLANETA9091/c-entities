# Baseline run: `mixed-50k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 50000 (items 34897 / stands 10136 / zombies 4967) (merge shrink: 41527 -> 41527) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 168.011 / 190.504 / 235.662 / 297.495 |
| MSPT mean, ms | 171.937 |
| TPS estimate (1000/p50) | 6.0 |
| heap used / max, MB | 1027.0 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.64% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 1.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 6 | 1.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.21% | `java.lang.ThreadLocal.get() line: 172` |
| 8 | 1.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.19% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 10 | 1.15% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 11 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 14 | 1.07% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 15 | 1.06% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |

