# Baseline run: `items-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62925 -> 62925) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 64.113 / 78.777 / 141.553 / 194.648 |
| MSPT mean, ms | 69.816 |
| TPS estimate (1000/p50) | 15.6 |
| heap used / max, MB | 2057.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.11% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.26% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 7 | 2.14% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 8 | 1.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 1.64% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 10 | 1.63% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.61% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 13 | 1.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.43% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 15 | 1.38% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

