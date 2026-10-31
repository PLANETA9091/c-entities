# Baseline run: `items-100k-mod`

Server: `26.3-235-bb10cf0 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62899 -> 62899) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.405 / 94.009 / 164.342 / 236.834 |
| MSPT mean, ms | 45.197 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1153.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 2 | 10.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5133` |
| 3 | 5.25% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.96% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 3.28% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 7 | 2.92% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8771` |
| 8 | 1.99% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5860` |
| 11 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.05% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 14 | 0.85% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8785` |
| 15 | 0.80% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |

