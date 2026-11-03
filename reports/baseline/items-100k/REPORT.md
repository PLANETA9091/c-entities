# Baseline run: `items-100k`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62931 -> 62931) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 63.990 / 79.544 / 150.903 / 192.842 |
| MSPT mean, ms | 70.811 |
| TPS estimate (1000/p50) | 15.6 |
| heap used / max, MB | 1361.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.57% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.20% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.61% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.23% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.04% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 9 | 1.97% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.86% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 13 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.38% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |

