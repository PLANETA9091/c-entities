# Baseline run: `items-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54774 -> 54774) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 41.354 / 65.287 / 155.074 / 350.486 |
| MSPT mean, ms | 49.907 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1231.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3190` |
| 2 | 9.11% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 6.04% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3053` |
| 4 | 5.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 5 | 4.97% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 4.39% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 3818` |
| 7 | 3.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 3.61% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 9 | 2.59% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2079` |
| 10 | 1.23% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 11 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 0.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2588` |
| 13 | 0.88% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1982` |

