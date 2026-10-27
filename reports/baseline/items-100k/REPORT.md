# Baseline run: `items-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62999 -> 62999) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 56.701 / 73.327 / 128.275 / 202.841 |
| MSPT mean, ms | 62.473 |
| TPS estimate (1000/p50) | 17.6 |
| heap used / max, MB | 1270.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.78% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 1.94% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 10 | 1.80% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.69% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 13 | 1.67% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.67% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 1.63% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |

