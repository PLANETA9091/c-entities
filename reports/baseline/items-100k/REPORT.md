# Baseline run: `items-100k`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62820 -> 62820) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 58.800 / 71.605 / 128.684 / 178.230 |
| MSPT mean, ms | 64.463 |
| TPS estimate (1000/p50) | 17.0 |
| heap used / max, MB | 1110.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 3 | 3.96% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 3.01% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 1.97% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 8 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.77% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 11 | 1.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 12 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 15 | 1.32% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |

