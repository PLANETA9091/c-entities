# Baseline run: `items-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63062 -> 63062) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 77.924 / 91.724 / 136.653 / 199.154 |
| MSPT mean, ms | 80.288 |
| TPS estimate (1000/p50) | 12.8 |
| heap used / max, MB | 1826.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 4.08% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 4.01% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 6 | 3.05% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 2.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 2.49% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 10 | 2.44% | `java.util.HashMap.getNode(Object) line: 585` |
| 11 | 2.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.81% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 14 | 1.56% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 15 | 1.54% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |

