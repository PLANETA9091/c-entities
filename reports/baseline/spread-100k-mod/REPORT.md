# Baseline run: `spread-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88762 -> 88650) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 76.852 / 133.347 / 251.055 / 317.037 |
| MSPT mean, ms | 90.978 |
| TPS estimate (1000/p50) | 13.0 |
| heap used / max, MB | 1976.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.33% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14243` |
| 2 | 6.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 6.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 4.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10077` |
| 5 | 4.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 4.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8984` |
| 7 | 3.52% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9619` |
| 8 | 2.36% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 2.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6211` |
| 11 | 1.74% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.35% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 13 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8700` |
| 14 | 1.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 1.28% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |

