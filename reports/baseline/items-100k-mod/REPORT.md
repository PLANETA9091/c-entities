# Baseline run: `items-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62844 -> 62844) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 24.680 / 59.432 / 121.336 / 198.620 |
| MSPT mean, ms | 33.846 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1395.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8042` |
| 2 | 7.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4360` |
| 3 | 5.83% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 5.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.49% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3055` |
| 6 | 4.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7989` |
| 7 | 3.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2947` |
| 8 | 2.77% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 9 | 2.65% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5080` |
| 11 | 1.92% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3008` |
| 12 | 1.51% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4366` |
| 15 | 1.00% | `net.minecraft.world.entity.Entity.getType() line: 633` |

