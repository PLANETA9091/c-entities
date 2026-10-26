# Baseline run: `items-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54837 -> 54837) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 13.437 / 17.683 / 79.592 / 110.691 |
| MSPT mean, ms | 15.752 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1186.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.24% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4002` |
| 2 | 10.62% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4198` |
| 4 | 5.02% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6454` |
| 5 | 4.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2076` |
| 6 | 2.96% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 2.34% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 1.87% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 9 | 1.76% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4042` |
| 10 | 1.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2030` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3489` |
| 12 | 1.31% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 13 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2105` |
| 15 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

