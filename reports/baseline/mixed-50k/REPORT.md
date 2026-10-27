# Baseline run: `mixed-50k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 50000 (items 34897 / stands 10136 / zombies 4967) (merge shrink: 41497 -> 41495) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 144.599 / 166.541 / 196.078 / 218.768 |
| MSPT mean, ms | 148.801 |
| TPS estimate (1000/p50) | 6.9 |
| heap used / max, MB | 1452.3 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.06% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 1.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 6 | 1.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 1.57% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 8 | 1.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 9 | 1.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.21% | `java.lang.ThreadLocal.get() line: 172` |
| 13 | 1.20% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 14 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 15 | 1.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

