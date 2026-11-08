# Baseline run: `items-100k`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62920 -> 62920) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 71.222 / 119.039 / 178.717 / 226.990 |
| MSPT mean, ms | 83.866 |
| TPS estimate (1000/p50) | 14.0 |
| heap used / max, MB | 2498.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 4 | 3.08% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 1.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.70% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 9 | 1.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 10 | 1.64% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.58% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.31% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 14 | 1.27% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 15 | 1.20% | `net.minecraft.world.entity.Entity.getX() line: 5100` |

