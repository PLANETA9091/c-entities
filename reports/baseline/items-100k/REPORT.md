# Baseline run: `items-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62891 -> 62891) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 56.506 / 71.800 / 128.866 / 229.998 |
| MSPT mean, ms | 62.132 |
| TPS estimate (1000/p50) | 17.7 |
| heap used / max, MB | 1587.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.09% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 6 | 2.78% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 7 | 2.41% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 2.19% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.96% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 14 | 1.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.63% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |

