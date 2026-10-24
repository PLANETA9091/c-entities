# Baseline run: `spread-100k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 77226 -> 77226) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 87.693 / 139.829 / 281.560 / 474.704 |
| MSPT mean, ms | 103.808 |
| TPS estimate (1000/p50) | 11.4 |
| heap used / max, MB | 1930.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 6.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 3 | 6.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 4.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.95% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 2.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 2.11% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 10 | 1.60% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 995` |
| 11 | 1.59% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.30% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 13 | 1.19% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.16% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 15 | 1.02% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

