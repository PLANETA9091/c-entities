# Baseline run: `items-100k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62867 -> 62867) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 62.851 / 81.351 / 143.950 / 197.576 |
| MSPT mean, ms | 68.434 |
| TPS estimate (1000/p50) | 15.9 |
| heap used / max, MB | 929.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.12% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.00% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 2.01% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.69% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 14 | 1.51% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 15 | 1.39% | `java.util.EnumMap.get(Object) line: 248` |

