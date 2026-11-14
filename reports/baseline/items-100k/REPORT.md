# Baseline run: `items-100k`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62970 -> 62970) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 69.759 / 110.432 / 178.413 / 244.752 |
| MSPT mean, ms | 80.616 |
| TPS estimate (1000/p50) | 14.3 |
| heap used / max, MB | 874.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 3.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 7 | 2.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 9 | 2.33% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 10 | 1.98% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 13 | 1.37% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 14 | 1.33% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 15 | 1.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

