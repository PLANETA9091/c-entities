# Baseline run: `items-100k`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62957 -> 62957) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 65.493 / 82.668 / 156.656 / 250.952 |
| MSPT mean, ms | 72.457 |
| TPS estimate (1000/p50) | 15.3 |
| heap used / max, MB | 984.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 2.85% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 5 | 2.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 6 | 2.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.27% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 8 | 2.18% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 1.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 11 | 1.90% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 12 | 1.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

