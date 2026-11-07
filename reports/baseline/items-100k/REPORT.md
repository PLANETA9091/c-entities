# Baseline run: `items-100k`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62996 -> 62996) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 69.034 / 122.721 / 188.090 / 230.763 |
| MSPT mean, ms | 82.121 |
| TPS estimate (1000/p50) | 14.5 |
| heap used / max, MB | 1916.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.07% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 4 | 2.72% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 6 | 2.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 2.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 1.84% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 9 | 1.73% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 10 | 1.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 13 | 1.25% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.20% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 15 | 1.18% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

