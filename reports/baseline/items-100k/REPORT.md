# Baseline run: `items-100k`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63017 -> 63017) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 67.010 / 112.576 / 165.475 / 251.279 |
| MSPT mean, ms | 79.374 |
| TPS estimate (1000/p50) | 14.9 |
| heap used / max, MB | 1633.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 3.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 2.45% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 6 | 2.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 1.99% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.49% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 1.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.34% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 13 | 1.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 14 | 1.15% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 15 | 1.04% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |

