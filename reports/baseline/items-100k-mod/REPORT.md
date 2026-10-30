# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62883 -> 62883) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 30.492 / 87.063 / 159.146 / 282.026 |
| MSPT mean, ms | 41.366 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1153.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4676` |
| 2 | 12.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8431` |
| 3 | 5.68% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8378` |
| 5 | 4.12% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.68% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3377` |
| 7 | 2.86% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 8 | 2.29% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3269` |
| 10 | 1.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5467` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4740` |
| 13 | 1.24% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.16% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 15 | 1.11% | `net.minecraft.world.entity.Entity.getType() line: 633` |

