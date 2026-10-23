# Baseline run: `mixed-150k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 101541 -> 99604) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 646.418 / 703.514 / 750.751 / 791.172 |
| MSPT mean, ms | 650.122 |
| TPS estimate (1000/p50) | 1.5 |
| heap used / max, MB | 2542.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.46% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 3 | 3.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 4 | 3.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 2.07% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2023` |
| 8 | 2.02% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 2.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 2.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 11 | 1.92% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1000` |
| 12 | 1.78% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 13 | 1.57% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 14 | 1.12% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 15 | 1.11% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |

