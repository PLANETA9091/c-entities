# Baseline run: `mixed-50k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 50000 (items 34897 / stands 10136 / zombies 4967) (merge shrink: 41491 -> 41491) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 142.177 / 159.626 / 203.885 / 216.936 |
| MSPT mean, ms | 145.585 |
| TPS estimate (1000/p50) | 7.0 |
| heap used / max, MB | 1530.7 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.72% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.18% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 2.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 6 | 1.76% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 1.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 8 | 1.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.53% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.48% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 11 | 1.32% | `java.util.HashMap.getNode(Object) line: 585` |
| 12 | 1.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.22% | `java.lang.ThreadLocal.get() line: 172` |
| 14 | 1.18% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 15 | 1.09% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |

