# Baseline run: `items-100k`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62939 -> 62939) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 61.350 / 77.667 / 139.919 / 173.315 |
| MSPT mean, ms | 67.338 |
| TPS estimate (1000/p50) | 16.3 |
| heap used / max, MB | 2275.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 3.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.33% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 5 | 2.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 8 | 2.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.94% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.91% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 12 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 13 | 1.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.10% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 15 | 1.07% | `java.util.EnumMap.get(Object) line: 248` |

