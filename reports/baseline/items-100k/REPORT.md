# Baseline run: `items-100k`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63047 -> 63047) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 54.024 / 68.594 / 126.434 / 164.590 |
| MSPT mean, ms | 59.776 |
| TPS estimate (1000/p50) | 18.5 |
| heap used / max, MB | 1433.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 5.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.52% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 5 | 3.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.46% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 8 | 2.27% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 9 | 2.01% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.87% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.61% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.55% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 14 | 1.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |

