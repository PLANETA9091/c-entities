# Baseline run: `items-100k-mod`

Server: `26.3-235-bb10cf0 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62953 -> 62953) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 30.974 / 73.662 / 133.301 / 168.454 |
| MSPT mean, ms | 39.677 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1384.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.08% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 2 | 7.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 3 | 7.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 4.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5145` |
| 6 | 3.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 7 | 2.21% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3723` |
| 8 | 2.19% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 9 | 2.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5874` |
| 10 | 1.96% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 11 | 1.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 12 | 1.44% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5151` |
| 14 | 1.33% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 15 | 1.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

