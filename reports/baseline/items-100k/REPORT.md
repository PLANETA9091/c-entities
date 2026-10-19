# Baseline run: `items-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62911 -> 62911) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 59.686 / 68.519 / 102.132 / 158.616 |
| MSPT mean, ms | 63.073 |
| TPS estimate (1000/p50) | 16.8 |
| heap used / max, MB | 2216.8 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 3 | 2.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 2.84% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 5 | 2.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 7 | 2.26% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 8 | 2.22% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 9 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.77% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 11 | 1.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.68% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 13 | 1.51% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 14 | 1.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 15 | 1.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 115` |

