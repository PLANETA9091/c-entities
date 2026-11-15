# Baseline run: `items-100k`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62936 -> 62936) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 71.463 / 113.023 / 180.151 / 248.629 |
| MSPT mean, ms | 82.899 |
| TPS estimate (1000/p50) | 14.0 |
| heap used / max, MB | 1872.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.02% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.16% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.78% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 9 | 1.56% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 10 | 1.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 11 | 1.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 1.08% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

