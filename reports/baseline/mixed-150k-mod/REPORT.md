# Baseline run: `mixed-150k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 99825 -> 99531) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 620.780 / 720.343 / 763.816 / 847.621 |
| MSPT mean, ms | 632.639 |
| TPS estimate (1000/p50) | 1.6 |
| heap used / max, MB | 2858.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.90% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 3 | 2.78% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 4 | 2.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.64% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1000` |
| 7 | 2.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.77% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 9 | 1.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 1.63% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 11 | 1.58% | `java.util.HashMap.getNode(Object) line: 585` |
| 12 | 1.45% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 13 | 1.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 14 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 15 | 1.19% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |

