# Baseline run: `mixed-150k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113959 -> 113910) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 109.679 / 236.949 / 405.119 / 755.134 |
| MSPT mean, ms | 139.587 |
| TPS estimate (1000/p50) | 9.1 |
| heap used / max, MB | 2284.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.00% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 7.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10077` |
| 3 | 4.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 4.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8934` |
| 5 | 3.91% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14243` |
| 6 | 3.78% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 7 | 3.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.16% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 9 | 2.14% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9619` |
| 10 | 1.51% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14417` |
| 11 | 1.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.41% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1288` |
| 13 | 1.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 14 | 0.95% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 0.94% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |

