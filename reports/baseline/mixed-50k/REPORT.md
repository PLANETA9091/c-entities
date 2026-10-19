# Baseline run: `mixed-50k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 50000 (items 34897 / stands 10136 / zombies 4967) (merge shrink: 41509 -> 41509) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 126.962 / 138.526 / 173.082 / 209.136 |
| MSPT mean, ms | 129.173 |
| TPS estimate (1000/p50) | 7.9 |
| heap used / max, MB | 1735.6 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.17% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 2.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 6 | 1.84% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.58% | `java.lang.ThreadLocal.get() line: 172` |
| 11 | 1.51% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 14 | 1.39% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 15 | 1.37% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |

