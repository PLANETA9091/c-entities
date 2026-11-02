# Baseline run: `items-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63015 -> 63015) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 62.660 / 84.342 / 150.225 / 203.796 |
| MSPT mean, ms | 70.337 |
| TPS estimate (1000/p50) | 16.0 |
| heap used / max, MB | 2321.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.37% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.00% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.51% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 5 | 2.43% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 2.29% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 7 | 2.19% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 8 | 2.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.73% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 14 | 1.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.60% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |

