# Baseline run: `items-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62835 -> 62835) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 27.014 / 88.862 / 166.492 / 270.849 |
| MSPT mean, ms | 39.701 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1240.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 2 | 8.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 3 | 5.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8056` |
| 4 | 4.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2961` |
| 6 | 2.95% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3022` |
| 7 | 2.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5094` |
| 8 | 2.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4374` |
| 9 | 1.89% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3069` |
| 10 | 1.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2190` |
| 11 | 1.55% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 12 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.31% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 15 | 1.27% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7699` |

