# Baseline run: `mixed-150k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 101643 -> 99715) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 688.630 / 746.255 / 796.543 / 867.764 |
| MSPT mean, ms | 688.901 |
| TPS estimate (1000/p50) | 1.5 |
| heap used / max, MB | 1878.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.90% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 4 | 2.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.45% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 2.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1000` |
| 8 | 2.15% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 2.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 1.84% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 11 | 1.59% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 12 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 13 | 1.33% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 14 | 1.33% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 15 | 1.20% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |

