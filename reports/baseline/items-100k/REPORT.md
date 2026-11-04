# Baseline run: `items-100k`

Server: `26.3-241-5050b0f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62911 -> 62911) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 66.403 / 84.304 / 149.618 / 199.906 |
| MSPT mean, ms | 72.987 |
| TPS estimate (1000/p50) | 15.1 |
| heap used / max, MB | 2038.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.96% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.45% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.23% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.70% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 6 | 2.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 1.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.78% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 11 | 1.73% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.72% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 13 | 1.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

