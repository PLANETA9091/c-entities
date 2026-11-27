# Baseline run: `items-100k`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62975 -> 62975) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 81.835 / 143.769 / 211.181 / 284.299 |
| MSPT mean, ms | 97.016 |
| TPS estimate (1000/p50) | 12.2 |
| heap used / max, MB | 2657.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.28% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 1.64% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 8 | 1.60% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 9 | 1.58% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 10 | 1.47% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 11 | 1.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 13 | 1.27% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.26% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 1.22% | `net.minecraft.world.entity.Entity.getX() line: 5100` |

