# Baseline run: `items-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 25000 / stands 0 / zombies 0) (merge shrink: 20031 -> 20031) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 21.787 / 26.842 / 37.623 / 54.130 |
| MSPT mean, ms | 22.633 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 948.9 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.78% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 2 | 3.61% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 3.44% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 3.41% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 2.64% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 2.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 2.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 9 | 1.77% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 10 | 1.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.50% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 12 | 1.44% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2189` |
| 13 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 15 | 1.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |

