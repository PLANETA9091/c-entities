# Baseline run: `items-100k`

Server: `26.3-235-bb10cf0 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62902 -> 62902) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 70.008 / 85.633 / 154.924 / 200.003 |
| MSPT mean, ms | 76.678 |
| TPS estimate (1000/p50) | 14.3 |
| heap used / max, MB | 1200.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.37% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.16% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.31% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 8 | 1.93% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 9 | 1.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.37% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 14 | 1.27% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 15 | 1.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |

