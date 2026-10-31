# Baseline run: `items-100k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62891 -> 62891) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 62.990 / 79.078 / 141.999 / 207.826 |
| MSPT mean, ms | 68.977 |
| TPS estimate (1000/p50) | 15.9 |
| heap used / max, MB | 1542.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.76% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 2.78% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 6 | 2.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 2.06% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 13 | 1.63% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.61% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.55% | `java.util.EnumMap.get(Object) line: 248` |

