# Baseline run: `items-100k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54751 -> 54751) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 15.163 / 19.224 / 76.868 / 122.915 |
| MSPT mean, ms | 17.088 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1452.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.99% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 10.47% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3625` |
| 3 | 6.76% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5422` |
| 4 | 5.96% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3624` |
| 5 | 4.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3806` |
| 6 | 2.76% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 1.68% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 9 | 1.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 10 | 1.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 11 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.17% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 13 | 1.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3114` |
| 15 | 1.00% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |

