# Baseline run: `items-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62959 -> 62959) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 58.927 / 73.602 / 135.651 / 212.009 |
| MSPT mean, ms | 65.013 |
| TPS estimate (1000/p50) | 17.0 |
| heap used / max, MB | 1218.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.33% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1883` |
| 2 | 7.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 4.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 4.09% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 6 | 2.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.22% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 8 | 2.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.12% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.66% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.61% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.40% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 14 | 1.33% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |

