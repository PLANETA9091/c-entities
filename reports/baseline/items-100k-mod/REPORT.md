# Baseline run: `items-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54816 -> 54816) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 16.591 / 19.470 / 93.415 / 110.368 |
| MSPT mean, ms | 18.779 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1162.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.82% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.09% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3314` |
| 3 | 8.46% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4210` |
| 4 | 7.17% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 5 | 4.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3460` |
| 6 | 2.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2058` |
| 7 | 2.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2146` |
| 9 | 1.86% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 10 | 1.79% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 11 | 1.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2020` |
| 12 | 1.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2084` |
| 13 | 1.18% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 14 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3341` |

