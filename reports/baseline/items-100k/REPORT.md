# Baseline run: `items-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62955 -> 62955) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 77.000 / 96.489 / 170.830 / 293.435 |
| MSPT mean, ms | 84.267 |
| TPS estimate (1000/p50) | 13.0 |
| heap used / max, MB | 2052.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.10% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 3 | 3.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 3.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.58% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.90% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.72% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 13 | 1.55% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.47% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.46% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |

