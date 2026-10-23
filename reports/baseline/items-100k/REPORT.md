# Baseline run: `items-100k`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62914 -> 62914) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 37.595 / 45.789 / 97.714 / 129.916 |
| MSPT mean, ms | 41.845 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1176.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.98% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 2.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 4 | 2.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 5 | 1.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.94% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 1.83% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 8 | 1.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.71% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.71% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 11 | 1.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 13 | 1.24% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 14 | 1.13% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.06% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |

