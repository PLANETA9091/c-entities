# Baseline run: `items-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62856 -> 62856) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 68.640 / 85.398 / 155.702 / 213.800 |
| MSPT mean, ms | 75.186 |
| TPS estimate (1000/p50) | 14.6 |
| heap used / max, MB | 1249.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.17% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 3.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.98% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.49% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.10% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 9 | 1.77% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 10 | 1.69% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 1.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 1.20% | `java.util.EnumMap.get(Object) line: 248` |

