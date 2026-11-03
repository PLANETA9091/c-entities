# Baseline run: `items-100k`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62942 -> 62942) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 76.058 / 98.095 / 172.396 / 214.747 |
| MSPT mean, ms | 83.462 |
| TPS estimate (1000/p50) | 13.1 |
| heap used / max, MB | 1490.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.30% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 3 | 4.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 3.02% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.70% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.83% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 10 | 1.78% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 15 | 1.23% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

