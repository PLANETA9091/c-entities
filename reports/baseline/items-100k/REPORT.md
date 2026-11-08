# Baseline run: `items-100k`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63076 -> 63076) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 56.212 / 96.734 / 144.395 / 188.584 |
| MSPT mean, ms | 66.958 |
| TPS estimate (1000/p50) | 17.8 |
| heap used / max, MB | 1577.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 4.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.09% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 5 | 2.68% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.63% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.51% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 8 | 2.23% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 9 | 2.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 2.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 2.04% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.77% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 13 | 1.69% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 1.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |

