# Baseline run: `items-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62956 -> 62956) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 76.779 / 94.526 / 181.523 / 225.752 |
| MSPT mean, ms | 83.577 |
| TPS estimate (1000/p50) | 13.0 |
| heap used / max, MB | 963.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 6.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.88% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.81% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.95% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 10 | 1.93% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.51% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.27% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |

