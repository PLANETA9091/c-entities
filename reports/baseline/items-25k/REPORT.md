# Baseline run: `items-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 25000 / stands 0 / zombies 0) (merge shrink: 19958 -> 19958) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 23.878 / 27.074 / 43.865 / 54.938 |
| MSPT mean, ms | 24.374 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1060.3 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 2 | 3.93% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 3 | 3.70% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 4 | 3.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.54% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.05% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 8 | 1.95% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 9 | 1.95% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 11 | 1.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 1.72% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 13 | 1.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 14 | 1.52% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 15 | 1.32% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2189` |

