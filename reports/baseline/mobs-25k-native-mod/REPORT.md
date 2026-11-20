# Baseline run: `mobs-25k-native-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25054 -> 25042) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 31.689 / 79.406 / 148.604 / 303.970 |
| MSPT mean, ms | 44.986 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1500.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.58% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.33% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.99% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 4 | 2.76% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14153` |
| 5 | 2.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9977` |
| 6 | 2.40% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 7 | 1.97% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14327` |
| 8 | 1.92% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9529` |
| 9 | 1.92% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 10 | 1.79% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 11 | 1.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.52% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 13 | 1.11% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1288` |
| 14 | 1.11% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 15 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

