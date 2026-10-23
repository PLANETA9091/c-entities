# Baseline run: `items-100k`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62921 -> 62921) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 60.775 / 78.300 / 145.824 / 170.901 |
| MSPT mean, ms | 67.240 |
| TPS estimate (1000/p50) | 16.5 |
| heap used / max, MB | 2346.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.93% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.90% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 6 | 2.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 1.98% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 11 | 1.89% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.84% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 13 | 1.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.34% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |

