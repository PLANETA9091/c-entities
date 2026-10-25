# Baseline run: `items-100k`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62906 -> 62906) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 58.298 / 73.464 / 131.328 / 180.499 |
| MSPT mean, ms | 63.632 |
| TPS estimate (1000/p50) | 17.2 |
| heap used / max, MB | 1286.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.90% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 4 | 3.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.02% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 2.00% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 10 | 1.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.84% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 12 | 1.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.73% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.67% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 15 | 1.63% | `java.util.EnumMap.get(Object) line: 248` |

