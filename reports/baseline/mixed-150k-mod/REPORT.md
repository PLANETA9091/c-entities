# Baseline run: `mixed-150k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 142139 -> 142139) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 158.606 / 214.670 / 318.902 / 393.294 |
| MSPT mean, ms | 174.684 |
| TPS estimate (1000/p50) | 6.3 |
| heap used / max, MB | 1739.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 6.01% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 5.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 4.77% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 3.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.91% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.89% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 2.04% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 1.87% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 10 | 1.80% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 11 | 1.72% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 12 | 1.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 13 | 1.61% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 1.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |

