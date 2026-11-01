# Baseline run: `items-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62924 -> 62924) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 68.928 / 84.034 / 159.314 / 201.091 |
| MSPT mean, ms | 75.395 |
| TPS estimate (1000/p50) | 14.5 |
| heap used / max, MB | 873.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.60% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 3.13% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.90% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.27% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 9 | 2.03% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.78% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 11 | 1.74% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.74% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 1.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |

