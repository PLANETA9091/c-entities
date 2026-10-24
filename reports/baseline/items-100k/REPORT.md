# Baseline run: `items-100k`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62901 -> 62901) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 56.801 / 73.072 / 131.267 / 181.670 |
| MSPT mean, ms | 62.136 |
| TPS estimate (1000/p50) | 17.6 |
| heap used / max, MB | 1622.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 4.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.65% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 4 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 1.99% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 7 | 1.84% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.57% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 11 | 1.55% | `net.minecraft.util.KeyframeTrackSampler.sample(long) line: 67` |
| 12 | 1.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 13 | 1.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.39% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 15 | 1.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |

