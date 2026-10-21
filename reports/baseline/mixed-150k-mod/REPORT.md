# Baseline run: `mixed-150k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100552 -> 100542) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 575.998 / 651.243 / 716.543 / 776.554 |
| MSPT mean, ms | 579.547 |
| TPS estimate (1000/p50) | 1.7 |
| heap used / max, MB | 1867.5 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 24.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.97% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.07% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 2.02% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.75% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.43% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 9 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 10 | 1.25% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 11 | 1.18% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.08% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 14 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 15 | 0.98% | `java.lang.ThreadLocal.get() line: 172` |

