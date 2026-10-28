# Baseline run: `items-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62930 -> 62930) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 62.382 / 77.448 / 143.401 / 234.052 |
| MSPT mean, ms | 68.669 |
| TPS estimate (1000/p50) | 16.0 |
| heap used / max, MB | 938.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.44% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 5 | 2.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 2.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.33% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 8 | 2.06% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 2.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 2.01% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.85% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.69% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.56% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |

