# Baseline run: `items-100k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63011 -> 63011) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 68.226 / 86.210 / 155.738 / 191.512 |
| MSPT mean, ms | 75.015 |
| TPS estimate (1000/p50) | 14.7 |
| heap used / max, MB | 1157.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.21% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.12% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 2.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.89% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.22% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 8 | 1.85% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.54% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 15 | 1.18% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

