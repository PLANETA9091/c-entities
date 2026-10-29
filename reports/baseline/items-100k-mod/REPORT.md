# Baseline run: `items-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62829 -> 62829) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 25.883 / 63.524 / 124.716 / 220.573 |
| MSPT mean, ms | 35.133 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1625.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8056` |
| 2 | 7.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4374` |
| 3 | 5.86% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 4.97% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3069` |
| 5 | 4.34% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 4.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 7 | 3.79% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 8 | 2.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2961` |
| 9 | 2.32% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.83% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3022` |
| 12 | 1.65% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2190` |
| 14 | 1.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5094` |
| 15 | 1.16% | `net.minecraft.world.entity.Entity.getType() line: 633` |

