# Baseline run: `items-100k-mod`

Server: `26.3-236-f119026 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62922 -> 62922) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 25.588 / 76.326 / 132.343 / 159.053 |
| MSPT mean, ms | 34.832 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1360.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5207` |
| 2 | 9.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8920` |
| 3 | 6.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 6.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3800` |
| 5 | 4.90% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8867` |
| 6 | 4.71% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 4.32% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3673` |
| 9 | 2.45% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 10 | 1.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5943` |
| 11 | 1.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.48% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3753` |
| 13 | 1.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 14 | 1.19% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5213` |

