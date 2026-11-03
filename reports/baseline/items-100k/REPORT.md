# Baseline run: `items-100k`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62963 -> 62963) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 65.991 / 84.103 / 144.375 / 193.442 |
| MSPT mean, ms | 72.488 |
| TPS estimate (1000/p50) | 15.2 |
| heap used / max, MB | 1287.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 4.02% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 3.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 1.71% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 8 | 1.67% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 9 | 1.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.38% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.13% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.06% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 15 | 1.05% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |

