# Baseline run: `items-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62907 -> 62907) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 64.885 / 77.843 / 147.124 / 188.886 |
| MSPT mean, ms | 70.902 |
| TPS estimate (1000/p50) | 15.4 |
| heap used / max, MB | 1372.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 5.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 4.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.76% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 2.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.86% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.84% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.77% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.59% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 15 | 1.25% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |

