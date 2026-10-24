# Baseline run: `items-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54806 -> 54806) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 47.366 / 70.361 / 203.349 / 235.422 |
| MSPT mean, ms | 56.350 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1344.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 2 | 10.08% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3027` |
| 3 | 9.60% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2927` |
| 4 | 6.26% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2898` |
| 5 | 4.78% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 3.33% | `jdk.internal.reflect.MethodHandleObjectFieldAccessorImpl.get(Object) line: 57` |
| 7 | 2.95% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 2.58% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 9 | 2.13% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2010` |
| 10 | 1.85% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 11 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2448` |
| 12 | 0.93% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2923` |
| 13 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1913` |
| 15 | 0.69% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |

