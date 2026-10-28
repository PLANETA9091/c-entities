# Baseline run: `items-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62944 -> 62944) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 62.258 / 77.485 / 142.255 / 187.116 |
| MSPT mean, ms | 68.548 |
| TPS estimate (1000/p50) | 16.1 |
| heap used / max, MB | 736.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.75% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1883` |
| 2 | 8.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 5.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 5.07% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.31% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.95% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 1.78% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.77% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 1.57% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.41% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 15 | 1.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |

