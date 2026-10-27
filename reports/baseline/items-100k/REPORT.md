# Baseline run: `items-100k`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62923 -> 62923) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 61.202 / 75.404 / 143.212 / 196.845 |
| MSPT mean, ms | 67.368 |
| TPS estimate (1000/p50) | 16.3 |
| heap used / max, MB | 1975.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.13% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 2.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 8 | 2.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 2.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.91% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.68% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.60% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 15 | 1.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

