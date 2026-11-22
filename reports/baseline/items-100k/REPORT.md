# Baseline run: `items-100k`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62952 -> 62952) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 70.820 / 134.825 / 198.780 / 267.470 |
| MSPT mean, ms | 85.941 |
| TPS estimate (1000/p50) | 14.1 |
| heap used / max, MB | 1638.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.52% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.12% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 7 | 2.10% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 8 | 1.96% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 1.84% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.30% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.18% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 15 | 1.14% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |

