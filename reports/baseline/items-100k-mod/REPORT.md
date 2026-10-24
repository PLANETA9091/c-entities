# Baseline run: `items-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54789 -> 54789) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 52.445 / 77.224 / 185.604 / 250.099 |
| MSPT mean, ms | 61.488 |
| TPS estimate (1000/p50) | 19.1 |
| heap used / max, MB | 1328.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 2 | 11.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3067` |
| 3 | 10.50% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2967` |
| 4 | 5.67% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2938` |
| 5 | 3.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 6 | 2.98% | `jdk.internal.reflect.MethodHandleObjectFieldAccessorImpl.get(Object) line: 57` |
| 7 | 2.85% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 2.70% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 9 | 1.94% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 10 | 1.85% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2050` |
| 11 | 1.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1953` |
| 12 | 1.07% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2963` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2488` |
| 15 | 0.85% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |

