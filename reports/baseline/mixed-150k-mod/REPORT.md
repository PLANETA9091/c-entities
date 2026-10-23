# Baseline run: `mixed-150k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 102031 -> 99966) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 574.770 / 644.719 / 729.385 / 1347.614 |
| MSPT mean, ms | 579.085 |
| TPS estimate (1000/p50) | 1.7 |
| heap used / max, MB | 2080.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.36% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 3.29% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 5 | 2.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 2.79% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 2.69% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1000` |
| 9 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 10 | 1.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 11 | 1.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.38% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 13 | 1.38% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 14 | 1.18% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 1.13% | `java.lang.ThreadLocal.get() line: 172` |

