# Baseline run: `items-100k`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63000 -> 63000) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 66.011 / 106.160 / 161.884 / 203.211 |
| MSPT mean, ms | 76.462 |
| TPS estimate (1000/p50) | 15.1 |
| heap used / max, MB | 943.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 2.90% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 5 | 2.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.78% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 10 | 1.93% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.64% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.63% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 13 | 1.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 1.36% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |

