# Baseline run: `items-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62923 -> 62923) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 60.859 / 74.959 / 142.325 / 222.980 |
| MSPT mean, ms | 66.264 |
| TPS estimate (1000/p50) | 16.4 |
| heap used / max, MB | 899.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.68% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 2.02% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 10 | 2.00% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.97% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.96% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.81% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 14 | 1.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.60% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

