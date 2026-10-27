# Baseline run: `items-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62901 -> 62901) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 64.956 / 79.823 / 140.416 / 201.514 |
| MSPT mean, ms | 71.194 |
| TPS estimate (1000/p50) | 15.4 |
| heap used / max, MB | 1209.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.99% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 4.07% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 2.79% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.64% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 2.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.76% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.67% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.50% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.31% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |

