# Baseline run: `mixed-150k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113898 -> 113871) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 115.108 / 234.094 / 449.391 / 803.381 |
| MSPT mean, ms | 145.065 |
| TPS estimate (1000/p50) | 8.7 |
| heap used / max, MB | 2410.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 8.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9987` |
| 3 | 4.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8844` |
| 4 | 3.96% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14153` |
| 5 | 3.81% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.53% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 7 | 3.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.70% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9529` |
| 9 | 1.66% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 10 | 1.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.30% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14327` |
| 12 | 1.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 1.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6153` |
| 14 | 1.12% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1288` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3078` |

