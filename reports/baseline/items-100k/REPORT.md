# Baseline run: `items-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63011 -> 63011) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 55.245 / 67.682 / 125.444 / 185.928 |
| MSPT mean, ms | 60.317 |
| TPS estimate (1000/p50) | 18.1 |
| heap used / max, MB | 1801.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.95% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.11% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.64% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.95% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 7 | 2.76% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 8 | 2.01% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 1.91% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.88% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.81% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.74% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 15 | 1.68% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |

