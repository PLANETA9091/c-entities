# Baseline run: `items-100k`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63025 -> 63025) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 69.434 / 112.223 / 168.850 / 235.372 |
| MSPT mean, ms | 81.511 |
| TPS estimate (1000/p50) | 14.4 |
| heap used / max, MB | 1212.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.08% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 3 | 4.07% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.09% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 11 | 1.55% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.51% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 13 | 1.46% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.39% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 15 | 1.38% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |

