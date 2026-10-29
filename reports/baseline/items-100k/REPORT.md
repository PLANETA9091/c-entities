# Baseline run: `items-100k`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62944 -> 62944) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 68.629 / 88.632 / 158.208 / 209.850 |
| MSPT mean, ms | 75.883 |
| TPS estimate (1000/p50) | 14.6 |
| heap used / max, MB | 1981.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.11% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 2.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.40% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 8 | 2.02% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 9 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.75% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.31% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 13 | 1.27% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.09% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

