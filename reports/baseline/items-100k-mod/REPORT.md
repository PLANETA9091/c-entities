# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62922 -> 62922) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.655 / 92.916 / 162.703 / 228.944 |
| MSPT mean, ms | 44.628 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1372.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4833` |
| 2 | 12.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8588` |
| 3 | 4.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.22% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 3.19% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3534` |
| 6 | 3.11% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 7 | 3.02% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8535` |
| 8 | 2.74% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3426` |
| 10 | 1.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2520` |
| 11 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5624` |
| 13 | 0.98% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4897` |
| 15 | 0.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |

