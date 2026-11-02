# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62970 -> 62970) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 26.223 / 70.711 / 138.009 / 178.164 |
| MSPT mean, ms | 36.435 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1627.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9126` |
| 2 | 8.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5413` |
| 3 | 6.07% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 4.66% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |
| 5 | 4.53% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4006` |
| 6 | 4.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 4.05% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 8 | 3.10% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 9 | 2.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3879` |
| 10 | 2.36% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 11 | 1.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 12 | 1.69% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3959` |
| 13 | 1.62% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6159` |
| 15 | 1.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |

