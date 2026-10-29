# Baseline run: `items-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62842 -> 62842) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 31.199 / 87.150 / 148.007 / 197.992 |
| MSPT mean, ms | 42.446 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1390.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8237` |
| 2 | 7.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4555` |
| 3 | 7.44% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.74% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 4.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8184` |
| 6 | 4.58% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.69% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3250` |
| 8 | 2.53% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3142` |
| 11 | 1.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5273` |
| 12 | 1.10% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2336` |
| 14 | 1.04% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

