# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62908 -> 62908) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.750 / 83.988 / 153.224 / 202.087 |
| MSPT mean, ms | 39.869 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1470.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8696` |
| 2 | 12.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4941` |
| 3 | 5.06% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.87% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8643` |
| 6 | 3.50% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 7 | 3.48% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3642` |
| 8 | 2.89% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3534` |
| 10 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5732` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5005` |
| 13 | 1.25% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 15 | 1.12% | `net.minecraft.world.entity.Entity.getId() line: 651` |

