# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62877 -> 62877) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 26.380 / 67.644 / 135.079 / 161.510 |
| MSPT mean, ms | 36.186 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1380.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4936` |
| 2 | 10.07% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8691` |
| 3 | 4.70% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.57% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8638` |
| 5 | 4.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3637` |
| 6 | 3.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3529` |
| 7 | 2.65% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 8 | 2.36% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.19% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3590` |
| 10 | 2.15% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 11 | 1.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2598` |
| 12 | 1.62% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5729` |
| 15 | 1.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |

