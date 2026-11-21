# Baseline run: `items-100k`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62955 -> 62955) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 75.437 / 127.659 / 201.466 / 260.753 |
| MSPT mean, ms | 89.271 |
| TPS estimate (1000/p50) | 13.3 |
| heap used / max, MB | 1219.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.70% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 3.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.17% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 6 | 2.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 2.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.48% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 9 | 1.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 10 | 1.39% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 11 | 1.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.24% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 15 | 1.13% | `net.minecraft.core.Vec3i.getY() line: 67` |

