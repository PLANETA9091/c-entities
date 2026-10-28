# Baseline run: `items-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62939 -> 62939) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 63.362 / 79.462 / 147.216 / 198.762 |
| MSPT mean, ms | 69.707 |
| TPS estimate (1000/p50) | 15.8 |
| heap used / max, MB | 1315.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.12% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.30% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 5.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 10 | 2.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.85% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.72% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.56% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |

