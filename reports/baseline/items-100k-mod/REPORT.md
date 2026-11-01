# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62858 -> 62858) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 27.168 / 72.641 / 144.104 / 170.689 |
| MSPT mean, ms | 36.695 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1528.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8920` |
| 2 | 7.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5207` |
| 3 | 6.08% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 5.60% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3800` |
| 5 | 4.74% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8867` |
| 6 | 4.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 4.13% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 8 | 2.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3673` |
| 9 | 2.27% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 10 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.75% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3753` |
| 12 | 1.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5213` |
| 13 | 1.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5953` |
| 14 | 1.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 15 | 1.33% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

