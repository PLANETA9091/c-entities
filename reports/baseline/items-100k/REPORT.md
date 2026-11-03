# Baseline run: `items-100k`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62900 -> 62900) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 54.706 / 70.375 / 124.865 / 180.443 |
| MSPT mean, ms | 59.801 |
| TPS estimate (1000/p50) | 18.3 |
| heap used / max, MB | 1929.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.87% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 2.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 2.23% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 2.05% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 12 | 2.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.92% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.61% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.55% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

