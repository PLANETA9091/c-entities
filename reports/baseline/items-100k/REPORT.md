# Baseline run: `items-100k`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62967 -> 62968) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 56.297 / 98.656 / 150.347 / 211.537 |
| MSPT mean, ms | 66.975 |
| TPS estimate (1000/p50) | 17.8 |
| heap used / max, MB | 798.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.67% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 8 | 2.31% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 9 | 2.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 2.16% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.78% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.71% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.70% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 14 | 1.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.30% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |

