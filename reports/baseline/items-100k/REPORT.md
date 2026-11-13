# Baseline run: `items-100k`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62967 -> 62966) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 66.419 / 111.737 / 169.186 / 203.853 |
| MSPT mean, ms | 78.145 |
| TPS estimate (1000/p50) | 15.1 |
| heap used / max, MB | 1936.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.15% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.10% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.07% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 2.90% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 2.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 8 | 1.63% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 10 | 1.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.47% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 12 | 1.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 13 | 1.25% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 14 | 1.17% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.16% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |

