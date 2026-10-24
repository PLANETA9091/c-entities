# Baseline run: `mixed-150k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 85506 -> 85506) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 105.349 / 218.806 / 373.456 / 583.204 |
| MSPT mean, ms | 131.646 |
| TPS estimate (1000/p50) | 9.5 |
| heap used / max, MB | 1361.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.89% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 2 | 6.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 5.69% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 4 | 4.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.96% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 3.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 2.03% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.62% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 10 | 1.53% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2479` |
| 11 | 1.48% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 12 | 1.10% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.03% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1948` |
| 14 | 1.02% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 995` |
| 15 | 0.98% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |

