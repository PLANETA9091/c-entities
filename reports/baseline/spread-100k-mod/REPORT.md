# Baseline run: `spread-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88755 -> 88619) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 83.438 / 152.476 / 290.992 / 394.786 |
| MSPT mean, ms | 100.740 |
| TPS estimate (1000/p50) | 12.0 |
| heap used / max, MB | 1958.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9987` |
| 3 | 3.67% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 4 | 3.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8894` |
| 5 | 3.20% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14153` |
| 6 | 2.64% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9529` |
| 9 | 1.67% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 10 | 1.39% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 11 | 1.12% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14327` |
| 12 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8630` |
| 13 | 1.07% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 14 | 1.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 0.98% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |

