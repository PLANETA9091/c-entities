# Baseline run: `items-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62997 -> 62997) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 36.583 / 100.461 / 164.459 / 202.441 |
| MSPT mean, ms | 53.355 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1491.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 5.27% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 3 | 4.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8934` |
| 4 | 4.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10077` |
| 5 | 4.01% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 3.47% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 7 | 2.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.90% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 2303` |
| 9 | 2.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 10 | 1.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6211` |
| 11 | 1.73% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14243` |
| 12 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6209` |
| 13 | 0.88% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 14 | 0.81% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 14054` |
| 15 | 0.79% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6485` |

