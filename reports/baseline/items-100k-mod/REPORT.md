# Baseline run: `items-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62892 -> 62892) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 22.829 / 57.212 / 121.295 / 212.799 |
| MSPT mean, ms | 31.612 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1707.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8102` |
| 2 | 8.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4420` |
| 3 | 7.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 4.62% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3115` |
| 5 | 4.48% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 4.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8049` |
| 7 | 3.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3007` |
| 8 | 3.43% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.44% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3068` |
| 10 | 1.99% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 11 | 1.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2228` |
| 12 | 1.84% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5140` |
| 14 | 1.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4426` |
| 15 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |

