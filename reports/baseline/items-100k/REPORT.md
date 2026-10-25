# Baseline run: `items-100k`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63057 -> 63057) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 66.492 / 81.587 / 151.104 / 187.020 |
| MSPT mean, ms | 73.011 |
| TPS estimate (1000/p50) | 15.0 |
| heap used / max, MB | 1756.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.39% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 4.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 3.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.15% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 8 | 2.11% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 1.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.95% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.64% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 14 | 1.50% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 15 | 1.12% | `java.util.EnumMap.get(Object) line: 248` |

