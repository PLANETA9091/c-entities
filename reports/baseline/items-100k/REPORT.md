# Baseline run: `items-100k`

Server: `26.3-259-c7097a9 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62915 -> 62915) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 74.325 / 117.374 / 185.098 / 268.806 |
| MSPT mean, ms | 86.473 |
| TPS estimate (1000/p50) | 13.5 |
| heap used / max, MB | 2517.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.93% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.57% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.06% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.75% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 9 | 1.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 10 | 1.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 13 | 1.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.14% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

