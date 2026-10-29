# Baseline run: `items-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62852 -> 62852) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 37.158 / 99.539 / 181.663 / 237.760 |
| MSPT mean, ms | 49.763 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1207.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8064` |
| 2 | 10.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4382` |
| 3 | 8.07% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.42% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8011` |
| 7 | 3.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3077` |
| 8 | 1.56% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5100` |
| 11 | 1.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8025` |
| 12 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.09% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2209` |
| 15 | 0.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4388` |

