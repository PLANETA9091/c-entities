# Baseline run: `items-100k`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63001 -> 63001) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 69.960 / 121.385 / 180.553 / 220.588 |
| MSPT mean, ms | 83.287 |
| TPS estimate (1000/p50) | 14.3 |
| heap used / max, MB | 1531.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.11% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 3 | 3.79% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.56% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 3.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 1.67% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 10 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 1.53% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.40% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 14 | 1.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 1.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

