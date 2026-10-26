# Baseline run: `items-100k`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62945 -> 62945) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 59.596 / 74.963 / 136.085 / 194.978 |
| MSPT mean, ms | 65.224 |
| TPS estimate (1000/p50) | 16.8 |
| heap used / max, MB | 1248.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.19% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 4.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.20% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 8 | 2.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 2.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.99% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.90% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.69% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.68% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 15 | 1.61% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |

