# Baseline run: `items-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54785 -> 54785) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 12.729 / 16.391 / 75.303 / 93.403 |
| MSPT mean, ms | 14.361 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1154.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4001` |
| 3 | 4.76% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 4 | 3.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 5 | 3.13% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2075` |
| 7 | 2.95% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 2.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2029` |
| 9 | 2.02% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 10 | 1.99% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 11 | 1.99% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 12 | 1.44% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4032` |
| 13 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3488` |
| 14 | 1.38% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4041` |
| 15 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |

