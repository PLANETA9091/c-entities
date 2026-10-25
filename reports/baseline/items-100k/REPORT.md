# Baseline run: `items-100k`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62951 -> 62951) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 64.501 / 79.902 / 144.166 / 207.471 |
| MSPT mean, ms | 70.097 |
| TPS estimate (1000/p50) | 15.5 |
| heap used / max, MB | 1068.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.98% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.70% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 5 | 3.01% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.19% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 8 | 2.12% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 9 | 1.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.34% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 12 | 1.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 13 | 1.27% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.23% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.22% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

