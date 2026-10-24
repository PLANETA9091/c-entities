# Baseline run: `items-100k`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62978 -> 62978) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 63.897 / 79.015 / 142.366 / 173.807 |
| MSPT mean, ms | 69.555 |
| TPS estimate (1000/p50) | 15.7 |
| heap used / max, MB | 1478.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.00% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 3 | 3.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.23% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 8 | 2.21% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 2.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 10 | 2.10% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 2.09% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 13 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.47% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

