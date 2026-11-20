# Baseline run: `items-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63015 -> 63014) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.221 / 76.889 / 180.389 / 339.239 |
| MSPT mean, ms | 48.180 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1233.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 6.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8844` |
| 3 | 5.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9987` |
| 4 | 4.04% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 5 | 3.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 3.27% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 2260` |
| 7 | 3.06% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 8 | 3.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1288` |
| 9 | 2.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6157` |
| 10 | 2.57% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 2.47% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14153` |
| 12 | 2.04% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6153` |
| 13 | 1.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6159` |
| 14 | 0.94% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9518` |
| 15 | 0.91% | `net.minecraft.world.level.ChunkPos.z() line: 20` |

