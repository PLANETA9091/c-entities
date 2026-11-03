# Baseline run: `items-100k`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62939 -> 62939) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 72.995 / 89.390 / 169.180 / 244.615 |
| MSPT mean, ms | 79.474 |
| TPS estimate (1000/p50) | 13.7 |
| heap used / max, MB | 1296.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 3.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.11% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 6 | 2.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.05% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 2.02% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 10 | 2.01% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.59% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 14 | 1.40% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.29% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

