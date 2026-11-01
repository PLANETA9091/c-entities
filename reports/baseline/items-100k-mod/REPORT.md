# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63014 -> 63014) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.592 / 88.641 / 142.480 / 210.332 |
| MSPT mean, ms | 40.078 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1778.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9008` |
| 2 | 7.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5295` |
| 3 | 5.45% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.75% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 4.12% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8955` |
| 7 | 3.22% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3888` |
| 8 | 2.69% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 2.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3761` |
| 11 | 1.35% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6031` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3841` |
| 14 | 1.00% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5301` |

