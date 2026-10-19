# Baseline run: `items-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 25000 / stands 0 / zombies 0) (merge shrink: 19981 -> 19981) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 26.150 / 31.547 / 45.485 / 52.275 |
| MSPT mean, ms | 27.293 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1565.0 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 2 | 4.34% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 4.18% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 3.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.73% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.57% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.20% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 9 | 1.95% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.33% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 13 | 1.30% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 14 | 1.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 15 | 1.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |

