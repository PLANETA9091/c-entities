# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62890 -> 62890) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 11.109 / 14.445 / 63.749 / 78.285 |
| MSPT mean, ms | 12.853 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1468.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 2 | 6.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.74% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 4 | 4.14% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 5 | 3.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 6 | 3.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2463` |
| 7 | 3.19% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 2.61% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 9 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2417` |
| 10 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2492` |
| 11 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3895` |
| 13 | 1.39% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 14 | 1.32% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

