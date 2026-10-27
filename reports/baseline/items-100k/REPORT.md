# Baseline run: `items-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62862 -> 62862) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 65.311 / 81.800 / 142.765 / 226.266 |
| MSPT mean, ms | 71.751 |
| TPS estimate (1000/p50) | 15.3 |
| heap used / max, MB | 1454.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 4.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.04% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 5 | 2.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 2.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 11 | 1.98% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.93% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.48% | `java.util.EnumMap.get(Object) line: 248` |

