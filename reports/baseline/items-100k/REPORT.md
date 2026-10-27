# Baseline run: `items-100k`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62897 -> 62897) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 64.051 / 77.229 / 145.628 / 212.485 |
| MSPT mean, ms | 70.093 |
| TPS estimate (1000/p50) | 15.6 |
| heap used / max, MB | 1273.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.01% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 3.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.48% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 8 | 2.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 2.23% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.93% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.80% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 13 | 1.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.45% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |

