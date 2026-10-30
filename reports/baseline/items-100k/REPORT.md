# Baseline run: `items-100k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62990 -> 62990) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 66.869 / 80.506 / 149.342 / 227.570 |
| MSPT mean, ms | 72.078 |
| TPS estimate (1000/p50) | 15.0 |
| heap used / max, MB | 1730.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.47% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 4.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.36% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.03% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.89% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.69% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.65% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 15 | 1.48% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |

