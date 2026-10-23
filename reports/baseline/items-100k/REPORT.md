# Baseline run: `items-100k`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62931 -> 62931) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 58.424 / 73.753 / 131.468 / 171.033 |
| MSPT mean, ms | 63.782 |
| TPS estimate (1000/p50) | 17.1 |
| heap used / max, MB | 1019.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 3.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 2.48% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 6 | 2.04% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 7 | 1.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.81% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 10 | 1.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.59% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.51% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 14 | 1.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.42% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

