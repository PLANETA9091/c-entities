# Baseline run: `items-100k`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62972 -> 62972) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 58.885 / 77.096 / 144.344 / 213.111 |
| MSPT mean, ms | 63.970 |
| TPS estimate (1000/p50) | 17.0 |
| heap used / max, MB | 933.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 5.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.34% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 4 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 7 | 2.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.27% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 9 | 2.01% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.00% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.86% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.78% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 13 | 1.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.37% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 15 | 1.34% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |

