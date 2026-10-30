# Baseline run: `items-100k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62937 -> 62937) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 63.292 / 78.663 / 139.669 / 174.525 |
| MSPT mean, ms | 69.630 |
| TPS estimate (1000/p50) | 15.8 |
| heap used / max, MB | 1651.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.12% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 3.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.02% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 2.91% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 6 | 2.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.08% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 10 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.81% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.77% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.40% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.37% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |

