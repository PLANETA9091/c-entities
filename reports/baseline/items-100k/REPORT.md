# Baseline run: `items-100k`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62871 -> 62871) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 64.434 / 78.674 / 143.333 / 194.184 |
| MSPT mean, ms | 70.212 |
| TPS estimate (1000/p50) | 15.5 |
| heap used / max, MB | 2075.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.39% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 4.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 2.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.55% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 8 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.39% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 2.34% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 2.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.58% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 13 | 1.48% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.47% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 15 | 1.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

