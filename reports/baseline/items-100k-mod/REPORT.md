# Baseline run: `items-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54812 -> 54812) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 9.746 / 13.454 / 65.746 / 95.934 |
| MSPT mean, ms | 11.523 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1155.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 5.22% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 3 | 3.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2032` |
| 4 | 3.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 3.43% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 3.23% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 7 | 2.51% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4004` |
| 8 | 2.34% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 9 | 2.09% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 10 | 2.06% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 11 | 1.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 12 | 1.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2107` |
| 13 | 1.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2078` |
| 14 | 1.48% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4044` |
| 15 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

