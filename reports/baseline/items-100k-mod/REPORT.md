# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62985 -> 62985) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 23.159 / 58.210 / 119.887 / 156.849 |
| MSPT mean, ms | 31.543 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1123.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5403` |
| 2 | 8.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9116` |
| 3 | 6.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 5.12% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3996` |
| 5 | 4.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9063` |
| 6 | 4.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.53% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 8 | 3.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3869` |
| 9 | 2.42% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 10 | 2.33% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3949` |
| 11 | 2.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 12 | 2.23% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 14 | 1.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5409` |
| 15 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |

