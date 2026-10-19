# Baseline run: `items-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63009 -> 63009) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 60.058 / 68.389 / 101.831 / 145.798 |
| MSPT mean, ms | 63.052 |
| TPS estimate (1000/p50) | 16.7 |
| heap used / max, MB | 2290.5 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 7.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 3 | 5.86% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 3.52% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 5 | 3.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.91% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 7 | 2.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 9 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.89% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 1.62% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 12 | 1.49% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 13 | 1.33% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 14 | 1.30% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 15 | 1.21% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |

