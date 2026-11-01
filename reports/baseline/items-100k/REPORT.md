# Baseline run: `items-100k`

Server: `26.3-236-f119026 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62962 -> 62962) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 62.141 / 76.579 / 138.945 / 177.928 |
| MSPT mean, ms | 67.871 |
| TPS estimate (1000/p50) | 16.1 |
| heap used / max, MB | 1151.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 5.32% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.05% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 2.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 2.22% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.09% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 2.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.79% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.34% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |

