# Baseline run: `items-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62996 -> 62996) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 44.959 / 56.722 / 118.476 / 159.448 |
| MSPT mean, ms | 50.257 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 939.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 2.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 4 | 2.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 6 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.94% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 8 | 1.65% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 10 | 1.58% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.55% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 12 | 1.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 13 | 1.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 14 | 1.32% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

