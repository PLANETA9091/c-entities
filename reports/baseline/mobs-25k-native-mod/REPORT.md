# Baseline run: `mobs-25k-native-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25059 -> 25048) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 24.919 / 65.378 / 125.042 / 236.328 |
| MSPT mean, ms | 36.137 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1611.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.87% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9619` |
| 3 | 2.79% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14243` |
| 4 | 2.56% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 5 | 2.25% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 6 | 2.14% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 7 | 1.97% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 8 | 1.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 9 | 1.69% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 10 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1288` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10067` |
| 14 | 1.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

