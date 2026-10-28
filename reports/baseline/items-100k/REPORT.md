# Baseline run: `items-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62909 -> 62909) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 64.708 / 81.086 / 145.993 / 194.931 |
| MSPT mean, ms | 70.738 |
| TPS estimate (1000/p50) | 15.5 |
| heap used / max, MB | 1066.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.25% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 2.11% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 2.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 2.04% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.85% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.48% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 14 | 1.44% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |

