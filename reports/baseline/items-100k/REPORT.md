# Baseline run: `items-100k`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62917 -> 62917) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 51.993 / 65.658 / 118.737 / 164.058 |
| MSPT mean, ms | 57.784 |
| TPS estimate (1000/p50) | 19.2 |
| heap used / max, MB | 1920.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.40% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.16% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.20% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 4 | 4.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.91% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 8 | 2.29% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 9 | 2.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 2.10% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.87% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.63% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.51% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 15 | 1.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |

