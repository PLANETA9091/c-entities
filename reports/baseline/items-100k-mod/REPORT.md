# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62964 -> 62964) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.401 / 74.383 / 155.972 / 335.649 |
| MSPT mean, ms | 44.171 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1054.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4962` |
| 2 | 11.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8717` |
| 3 | 5.05% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.20% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.58% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 6 | 3.12% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3663` |
| 7 | 3.12% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8664` |
| 8 | 3.10% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3537` |
| 10 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5026` |
| 11 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 0.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 13 | 0.92% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2601` |
| 14 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.80% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |

