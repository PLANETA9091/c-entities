# Baseline run: `items-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62918 -> 62918) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 61.044 / 77.593 / 140.068 / 178.835 |
| MSPT mean, ms | 66.679 |
| TPS estimate (1000/p50) | 16.4 |
| heap used / max, MB | 1084.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.44% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.41% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 2.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.88% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.58% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.50% | `java.util.EnumMap.get(Object) line: 248` |

