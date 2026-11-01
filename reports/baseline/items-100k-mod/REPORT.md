# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62980 -> 62980) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 25.094 / 61.828 / 134.234 / 180.575 |
| MSPT mean, ms | 33.878 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1521.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8988` |
| 2 | 8.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5275` |
| 3 | 6.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 5.11% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3868` |
| 5 | 4.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8935` |
| 6 | 4.25% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3741` |
| 8 | 2.69% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 2.17% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3821` |
| 10 | 2.13% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 11 | 2.06% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5281` |
| 14 | 1.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6021` |
| 15 | 1.33% | `net.minecraft.world.entity.Entity.getType() line: 633` |

