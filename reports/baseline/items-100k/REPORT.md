# Baseline run: `items-100k`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62882 -> 62882) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 72.837 / 119.937 / 184.228 / 226.480 |
| MSPT mean, ms | 85.364 |
| TPS estimate (1000/p50) | 13.7 |
| heap used / max, MB | 1570.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.19% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 3 | 3.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 4 | 3.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 3.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 1.58% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 10 | 1.56% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 11 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.51% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.43% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 14 | 1.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.40% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

