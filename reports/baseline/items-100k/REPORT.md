# Baseline run: `items-100k`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62985 -> 62985) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 60.410 / 76.624 / 141.777 / 196.386 |
| MSPT mean, ms | 66.431 |
| TPS estimate (1000/p50) | 16.6 |
| heap used / max, MB | 892.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.84% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 4 | 4.12% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 8 | 2.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.07% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.80% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 13 | 1.70% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 14 | 1.65% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

