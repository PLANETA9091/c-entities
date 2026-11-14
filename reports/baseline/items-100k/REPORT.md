# Baseline run: `items-100k`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63089 -> 63089) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 87.565 / 150.017 / 230.301 / 308.848 |
| MSPT mean, ms | 102.129 |
| TPS estimate (1000/p50) | 11.4 |
| heap used / max, MB | 1717.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.94% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 5 | 3.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.23% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 8 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.53% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 10 | 1.52% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 11 | 1.52% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.26% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 15 | 1.23% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

