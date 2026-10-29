# Baseline run: `items-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62944 -> 62944) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 65.471 / 83.162 / 152.382 / 210.840 |
| MSPT mean, ms | 72.657 |
| TPS estimate (1000/p50) | 15.3 |
| heap used / max, MB | 1086.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.37% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 5 | 2.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.20% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.14% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 9 | 1.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.93% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 11 | 1.79% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 1.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.37% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.14% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

