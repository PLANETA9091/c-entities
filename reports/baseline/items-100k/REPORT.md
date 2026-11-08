# Baseline run: `items-100k`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62924 -> 62924) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 76.553 / 128.646 / 197.606 / 306.596 |
| MSPT mean, ms | 89.797 |
| TPS estimate (1000/p50) | 13.1 |
| heap used / max, MB | 1807.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.99% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.79% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 1.90% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 8 | 1.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.48% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 10 | 1.35% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.19% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.14% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 14 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 1.06% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |

