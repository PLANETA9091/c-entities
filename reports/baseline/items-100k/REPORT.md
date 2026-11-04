# Baseline run: `items-100k`

Server: `26.3-239-13565bc (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62964 -> 62964) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 67.212 / 84.006 / 156.095 / 212.078 |
| MSPT mean, ms | 73.859 |
| TPS estimate (1000/p50) | 14.9 |
| heap used / max, MB | 797.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 6.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.13% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 2.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.54% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.23% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.05% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 9 | 1.78% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 10 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 15 | 1.25% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

