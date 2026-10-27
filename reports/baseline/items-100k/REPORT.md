# Baseline run: `items-100k`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62942 -> 62942) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 65.032 / 80.577 / 146.303 / 209.483 |
| MSPT mean, ms | 71.210 |
| TPS estimate (1000/p50) | 15.4 |
| heap used / max, MB | 1513.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.09% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 3.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 2.91% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 5 | 2.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 2.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.09% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 11 | 1.95% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.91% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 13 | 1.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.37% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.31% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

