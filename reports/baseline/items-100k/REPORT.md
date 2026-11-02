# Baseline run: `items-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62918 -> 62918) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 58.090 / 71.034 / 134.406 / 172.277 |
| MSPT mean, ms | 63.151 |
| TPS estimate (1000/p50) | 17.2 |
| heap used / max, MB | 1801.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.07% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.53% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.54% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 7 | 2.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.14% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 2.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 2.01% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.99% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.99% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.82% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.56% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

