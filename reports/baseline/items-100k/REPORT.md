# Baseline run: `items-100k`

Server: `26.3-239-13565bc (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62933 -> 62933) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 73.262 / 89.264 / 159.289 / 182.815 |
| MSPT mean, ms | 79.968 |
| TPS estimate (1000/p50) | 13.6 |
| heap used / max, MB | 781.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.45% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 3 | 4.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.68% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.38% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 8 | 2.13% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 9 | 1.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.42% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.34% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 14 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 1.02% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |

