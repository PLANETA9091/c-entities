# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62990 -> 62990) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.112 / 77.695 / 136.790 / 233.454 |
| MSPT mean, ms | 39.151 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1145.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 2 | 11.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4737` |
| 3 | 5.02% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.97% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 6 | 3.13% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 7 | 3.11% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3438` |
| 8 | 2.37% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3330` |
| 11 | 1.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5528` |
| 12 | 1.46% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4801` |
| 14 | 1.22% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.12% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |

